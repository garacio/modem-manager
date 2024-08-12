use std::io::{Stdout, stdout};
use std::sync::{mpsc};
use std::{io, thread};
use std::process::exit;
use std::time::{Duration, Instant};
use crossterm::event::poll;
use ratatui::{
    backend::{CrosstermBackend},
    buffer::Buffer,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    Terminal,
    text::Line,
    widgets::{Widget},
    Frame
};
use ratatui::widgets::Tabs;
use crate::modem_tools::modem::{get_modem_info, get_modem_info_string};
use crate::display_tools::tui::port_choice_menu::show_port_choice_menu;
use crate::display_tools::tui::app_tabs::{AppTabs, BandsSelectorActive, SelectedTab};
use strum::IntoEnumIterator;
use crate::display_tools::tui::errors;
use crate::modem_tools::supported_modems::Modem;

#[derive(Default, Clone)]
pub struct App {
    pub(crate) app_tabs: AppTabs,
    pub(crate) port_name: String,
    pub(crate) baud_rate: u32,
    pub(crate) exit: bool
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;


impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        self.baud_rate = 115_200;
        self.port_name = show_port_choice_menu(terminal)?;
        if self.port_name == "" {
            self.exit = true
        }

        let port_name = self.port_name.clone();
        let baud_rate = self.baud_rate.clone();

        self.app_tabs.active_bands_selector = BandsSelectorActive::UMTSBandsSelector;
        self.app_tabs.umts_bands_list_state.select(Some(0));
        self.app_tabs.lte_bands_list_state.select(None);

        let (_tx, rx) = mpsc::channel::<()>();

        let poll_interval = Duration::from_secs(3);
        let mut last_poll_time = Instant::now();

        self.update_modem_info();
        let modem_info = self.app_tabs.modem_info.clone();

        {
            let mi = modem_info.lock().unwrap();
            self.app_tabs.modem_capabilities.spec = Some(Modem::new(mi.model.as_str()).unwrap());
            self.app_tabs.config_umts_bands = mi.enabled_umts_bands.clone();
            self.app_tabs.config_lte_bands = mi.enabled_lte_bands.clone();
        }
        let _update_handle = thread::spawn(move || {
            loop {
                if rx.try_recv().is_ok() {
                    break;
                }

                if last_poll_time.elapsed() >= poll_interval {
                    let modem_info_string = get_modem_info_string(port_name.as_str(), baud_rate).unwrap_or_else(|err| {
                        eprintln!("{}", err);
                        String::new()
                    });

                    if modem_info_string.is_empty() {
                        thread::sleep(Duration::from_secs(5));
                        break
                    }
                    let updated_info = get_modem_info(modem_info_string).unwrap();
                    {
                        let mut info = modem_info.lock().unwrap();
                        *info = updated_info;
                    }
                    last_poll_time = Instant::now();
                }

                thread::sleep(Duration::from_millis(100));
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            if poll(Duration::from_millis(200))? {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn update_modem_info(&self) {
        let modem_info_string = get_modem_info_string(self.port_name.as_str(), self.baud_rate).unwrap_or_else(|err| {
                        eprintln!("{}", err);
                        exit(1);
                    });

        let updated_info = get_modem_info(modem_info_string).unwrap();

        {
            let modem_info = self.app_tabs.modem_info.clone();
            let mut info = modem_info.lock().unwrap();
            *info = updated_info;
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    pub fn exit(&mut self) {
        self.exit = true
    }

    pub(crate) fn next_tab(&mut self) {
        self.app_tabs.selected_tab = self.app_tabs.next();
    }

    pub(crate) fn _previous_tab(&mut self) {
        self.app_tabs.selected_tab = self.app_tabs._previous();
    }

    pub(crate) fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.app_tabs.cursor_index.saturating_sub(1);
        self.app_tabs.cursor_index = self.clamp_cursor(cursor_moved_left);
        self.app_tabs.cursor_position.x = self.app_tabs.cursor_index as u16 + 1;
    }

    pub(crate) fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.app_tabs.cursor_index.saturating_add(1);
        self.app_tabs.cursor_index = self.clamp_cursor(cursor_moved_right);
        self.app_tabs.cursor_position.x = self.app_tabs.cursor_index as u16 + 1;
    }

    fn byte_index(&self) -> usize {
        self.app_tabs.terminal_data.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.app_tabs.cursor_index)
            .unwrap_or(self.app_tabs.terminal_data.input.len())
    }

    pub(crate) fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.app_tabs.terminal_data.input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub(crate) fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.app_tabs.cursor_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.app_tabs.cursor_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.app_tabs.terminal_data.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.app_tabs.terminal_data.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.app_tabs.terminal_data.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.app_tabs.terminal_data.input.chars().count())
    }

    // fn reset_cursor(&mut self) {
    //     self.app_tabs.cursor_index = 0;
    // }


}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [_tabs_area, title_area] = horizontal.areas(header_area);

        // Render tabs
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.app_tabs.selected_tab.palette().c700);
        let selected_tab_index = self.app_tabs.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);

        self.app_tabs.clone().render(inner_area, buf);


        "Fibocom L8[5,6]0-GL".bold().render(title_area, buf);
        Line::raw("◄ ► or Tab to change tab| F10 to save | q or Esc to quit without saving")
        .centered()
        .render(footer_area, buf);
    }
}


/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
pub fn run_app() -> io::Result<()> {
    errors::install_hooks().expect("TODO: panic message");
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    restore()?;
    app_result
}
