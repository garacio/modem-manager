use std::sync::{Arc, Mutex};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind;
use ratatui::style::Stylize;
use ratatui::widgets::{ListState, Widget};
use ratatui::text::{Line, Span};
use strum::{Display, EnumCount, EnumIter, FromRepr};
use crate::modem_tools::types::ModemInfo;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, EnumCount)]
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

#[derive(Default, Clone)]
pub struct CursorPosition {
    pub x: u16,
    pub y: u16
}

#[derive(Default, Clone)]
pub struct TerminalData {
    pub input: String,
    pub output: String
}

#[derive(Default, Clone)]
pub struct AppTabs {
    pub selected_tab: SelectedTab,
    pub modem_info: Arc<Mutex<ModemInfo>>,
    pub band_list_state: Arc<Mutex<ListState>>,
    pub editing_mode: bool,
    pub cursor_index: usize,
    pub cursor_position: CursorPosition,
    pub terminal_data: TerminalData,
}
impl AppTabs {

    /// Get the previous tab, if there is no previous tab return the current tab.
    pub(crate) fn previous(&mut self) -> SelectedTab {
        let current_index: usize = self.selected_tab as usize;
        let previous_index = match current_index {
            0 => SelectedTab::COUNT.saturating_sub(1),
            _ => current_index.saturating_sub(1),
        };
        SelectedTab::from_repr(previous_index).unwrap_or(SelectedTab::from_repr(SelectedTab::COUNT.saturating_sub(1)).unwrap())
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub(crate) fn next(&mut self) -> SelectedTab {
        let current_index = self.selected_tab as usize;
        let next_index = match current_index {
            i if i == SelectedTab::COUNT.saturating_sub(1) => 0,
            _ => current_index.saturating_add(1)
        };
        SelectedTab::from_repr(next_index).unwrap_or(SelectedTab::from_repr(0).unwrap())
    }
}

impl Widget for AppTabs {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self.selected_tab {
            SelectedTab::MonitorTab => self.render_monitor_tab(area, buf),
            SelectedTab::BandsConfigTab => self.render_config_tab(area, buf),
            SelectedTab::TerminalTab => self.render_terminal_tab(area, buf),
        }
    }
}