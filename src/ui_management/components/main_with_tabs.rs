use std::default::Default;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Stylize};
use ratatui::style::palette::tailwind;
use ratatui::Frame;
use ratatui::text::Text;
use ratatui::widgets::{ListState, Paragraph, Tabs, Wrap};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter, FromRepr};
use crate::modem_tools::modem::{modem_execute, save_bands_command};
use crate::modem_tools::supported_modems::{Modem, ModemSpecs};
use crate::state_store::state::{ModemInfo, State};
use crate::ui_management::components::ComponentRender;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, EnumCount, Debug)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Signal monitor")]
    MonitorTab,
    #[strum(to_string = "Select bands")]
    BandsConfigTab,
    #[strum(to_string = "Terminal")]
    TerminalTab,
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::MonitorTab => tailwind::EMERALD,
            Self::BandsConfigTab => tailwind::INDIGO,
            Self::TerminalTab => tailwind::RED,
        }
    }
}
#[derive(Clone, Debug)]
pub struct CursorPosition {
    pub x: u16,
    pub _y: u16
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self{
            x: 1,
            _y: 0
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum BandsSelectorActive {
    #[default]
    UMTSBandsSelector,
    LTEBandsSelector,
}

#[derive(Default, Debug)]
pub(crate) struct BandsConfig {
    pub(crate) loaded: bool,
    pub(crate) umts: Vec<usize>,
    pub(crate) lte: Vec<usize>,
}

#[derive(Default, Debug)]
pub struct Props {
    pub(crate) error_message: Option<String>,
    pub(crate) port_name: String,
    pub(crate) baud_rate: u32,
    pub(crate) modem_info: ModemInfo,
    pub(crate) terminal_user_output: String,
    pub(crate) save_bands_command: String,
    pub(crate) modem_capabilities: ModemSpecs,
}

impl Props {
    pub fn from(state: State) -> Self {
        Props {
            error_message: None,
            port_name: state.port_name,
            baud_rate: state.baud_rate,
            modem_info: state.modem_info.clone(),
            terminal_user_output: state.terminal_user_output.clone(),
            save_bands_command: "".to_string(),
            modem_capabilities: Modem::new(state.modem_info.model.as_str()).unwrap(),
        }
    }

}

#[derive(Debug)]
pub struct MainWithTabs {
    pub(crate) selected_tab: SelectedTab,
    pub(crate) active_bands_selector: BandsSelectorActive,
    pub(crate) umts_bands_list_state: ListState,
    pub(crate) lte_bands_list_state: ListState,
    pub(crate) bands_config: BandsConfig,
    pub(crate) save_response: String,
    pub(crate) editing_mode: bool,
    pub(crate) cursor_index: usize,
    pub(crate) cursor_position: CursorPosition,
    pub(crate) terminal_user_input: String,
    pub(crate) terminal_user_output: Vec<String>,
    pub(crate) props: Props

}

impl MainWithTabs {
    pub(crate) fn new(state: &State) -> Self
    where
        Self: Sized
    {
        let props = Props::from(state.clone());
        Self {
            selected_tab: SelectedTab::default(),
            active_bands_selector: Default::default(),
            umts_bands_list_state: Default::default(),
            lte_bands_list_state: Default::default(),
            bands_config: Default::default(),
            save_response: "".to_string(),
            editing_mode: false,
            cursor_index: 0,
            cursor_position: Default::default(),
            terminal_user_input: "".to_string(),
            terminal_user_output: Default::default(),
            props
        }
    }

    pub(crate) fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized
    {
        let mut terminal_user_output = self.terminal_user_output.clone();
        if self.props.terminal_user_output != ""{
            terminal_user_output.push(self.props.terminal_user_output);
        }
        MainWithTabs {
            props: Props::from(state.clone()),
            terminal_user_output,
            ..self
        }
    }

    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn _previous_tab(&mut self) {
        let current_index: usize = self.selected_tab as usize;
        let previous_index = match current_index {
            0 => SelectedTab::COUNT.saturating_sub(1),
            _ => current_index.saturating_sub(1),
        };
        self.selected_tab = SelectedTab::from_repr(previous_index).unwrap_or(SelectedTab::from_repr(SelectedTab::COUNT.saturating_sub(1)).unwrap())
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub(crate) fn next_tab(&mut self) {
        let current_index = self.selected_tab as usize;
        let next_index = match current_index {
            i if i == SelectedTab::COUNT.saturating_sub(1) => 0,
            _ => current_index.saturating_add(1)
        };
        self.selected_tab = SelectedTab::from_repr(next_index).unwrap_or(SelectedTab::from_repr(0).unwrap())
    }

    pub fn handle_event(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Tab => self.next_tab(),
                KeyCode::Backspace => {
                    match self.selected_tab {
                        SelectedTab::TerminalTab => if self.editing_mode { self.delete_char() },
                        _ => {}
                    }
                }
                KeyCode::Char(ch) => {
                    match self.selected_tab {
                        SelectedTab::TerminalTab => if ch == 'i' && !self.editing_mode {
                                self.editing_mode = true
                            } else if self.editing_mode { self.enter_char(ch) },
                        SelectedTab::BandsConfigTab => if ch == ' ' {
                            match self.active_bands_selector {
                                BandsSelectorActive::UMTSBandsSelector => {
                                    let current_selection = self.umts_bands_list_state.selected().unwrap_or(0);
                                    let band = self.props.modem_capabilities.supported_umts_bands[current_selection];

                                    if self.bands_config.umts.contains(&band) {
                                        self.bands_config.umts.retain(|&x| x != band);
                                    } else {
                                        self.bands_config.umts.push(band);
                                    }
                                },
                                BandsSelectorActive::LTEBandsSelector => {
                                    let current_selection = self.lte_bands_list_state.selected().unwrap_or(0);
                                    let band = self.props.modem_capabilities.supported_lte_bands[current_selection];

                                    if self.bands_config.lte.contains(&band) {
                                        self.bands_config.lte.retain(|&x| x != band);
                                    } else {
                                        self.bands_config.lte.push(band);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Right => {
                    match self.selected_tab {
                        SelectedTab::TerminalTab => if self.editing_mode { self.move_cursor_right() }
                        SelectedTab::BandsConfigTab => self.toggle_band_selector(),
                        _ => {}
                    }
                }
                KeyCode::Left => {
                    match self.selected_tab {
                        SelectedTab::TerminalTab => if self.editing_mode { self.move_cursor_left() },
                        SelectedTab::BandsConfigTab => self.toggle_band_selector(),
                        _ => {}
                    }
                }
                KeyCode::Up => if let SelectedTab::BandsConfigTab = self.selected_tab {
                    match self.active_bands_selector {
                        BandsSelectorActive::UMTSBandsSelector => self.umts_bands_list_state.select_previous(),
                        BandsSelectorActive::LTEBandsSelector => self.lte_bands_list_state.select_previous(),
                    }
                }
            KeyCode::Down => if let SelectedTab::BandsConfigTab = self.selected_tab {
                match self.active_bands_selector {
                    BandsSelectorActive::UMTSBandsSelector => {
                        let current_selection = self.umts_bands_list_state.selected().unwrap_or(0);
                        if current_selection < self.props.modem_capabilities.supported_umts_bands.len()-1 {
                            self.umts_bands_list_state.select_next()
                        }
                    },
                    BandsSelectorActive::LTEBandsSelector => {
                        let current_selection = self.lte_bands_list_state.selected().unwrap_or(0);
                        if current_selection < self.props.modem_capabilities.supported_lte_bands.len()-1 {
                            self.lte_bands_list_state.select_next()
                        }
                    }
                }
            }
            KeyCode::F(10) => {
                let command = save_bands_command(self.bands_config.umts.clone(), self.bands_config.lte.clone());
                self.save_response = modem_execute(&self.props.port_name, &self.props.baud_rate, command.as_str()).unwrap_or_else(
                            |err| {
                                format!("{:?}", err)
                            }
                        );
            }
                _ => {}
            }
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
    pub show_cursor: bool,
}

impl ComponentRender<RenderProps> for MainWithTabs {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(props.area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [_tabs_area, title_area] = horizontal.areas(header_area);

        // Render tabs
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        let tabs = Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ");

        frame.render_widget(tabs, _tabs_area);

        let header = Paragraph::new(Text::from("Fibocom L8[5,6]0-GL"));
        frame.render_widget(header, title_area);

        let help_line = Line::raw("Tab to change tab| ◄ ► to select bands | i to enter editing mode | F10 to save | Esc to quit without saving")
        .centered();
        frame.render_widget(help_line, footer_area);

        let mut content_area = inner_area;

        #[cfg(debug_assertions)]
        {
            let tabs_content_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(inner_area);


            let debug_line = Paragraph::new(format!("{self:#?}"))
                .wrap(Wrap { trim: false });
            frame.render_widget(debug_line, tabs_content_area[1]);
            content_area = tabs_content_area[0]

        }

        match self.selected_tab {
            SelectedTab::MonitorTab => self.render_monitor_tab(frame, content_area),
            SelectedTab::BandsConfigTab => self.render_config_tab(frame, content_area),
            SelectedTab::TerminalTab => self.render_terminal_tab(frame, content_area)
        }


    }
}