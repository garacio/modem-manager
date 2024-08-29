use std::collections::HashSet;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph};
use crate::ui_management::components::main_with_tabs::{BandsSelectorActive, MainWithTabs};
use crate::modem_tools::modem::save_bands_command;

impl MainWithTabs {

    pub(crate) fn toggle_band_selector(&mut self) {
        match self.active_bands_selector {
            BandsSelectorActive::UMTSBandsSelector => self.active_bands_selector = BandsSelectorActive::LTEBandsSelector,
            BandsSelectorActive::LTEBandsSelector => self.active_bands_selector = BandsSelectorActive::UMTSBandsSelector
        }
    }
    pub(crate) fn render_config_tab(&mut self, frame: &mut Frame, area: Rect) {
        let mut umts_bands_list_state = self.umts_bands_list_state.clone();
        let mut lte_bands_list_state = self.lte_bands_list_state.clone();

        let tab_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90),
                Constraint::Percentage(10)
            ].as_ref()
            ).split(area);

        let bands_lists_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(tab_area[0]);

        if !self.bands_config.loaded {
            self.bands_config.lte = self.props.modem_info.enabled_lte_bands.clone();
            self.bands_config.umts = self.props.modem_info.enabled_umts_bands.clone();
            self.bands_config.loaded = true;
        }

        let enabled_lte_bands: HashSet<_> =  self.bands_config.lte.iter().collect();
        let enabled_umts_bands: HashSet<_> = self.bands_config.umts.iter().collect();

        let modem_caps = self.props.modem_capabilities.clone();

        let (umts_style, lte_style) = match self.active_bands_selector {
            BandsSelectorActive::UMTSBandsSelector => {
                (Style::default().fg(Color::White), Style::default().fg(Color::DarkGray))
            },
            BandsSelectorActive::LTEBandsSelector => {
                (Style::default().fg(Color::DarkGray), Style::default().fg(Color::White))
            }
        };

        let umts_bands_list = List::new(
                modem_caps.supported_umts_bands.iter().map(|b| {
                    let band_str = format!("B{}", b);
                    let checkbox = if enabled_umts_bands.contains(&b) {
                        "[x]"
                    } else {
                        "[ ]"
                    };
                    ListItem::new(format!("{} {}", checkbox, band_str))
                }).collect::<Vec<_>>()
            )
            .block(Block::bordered().borders(Borders::ALL).title("UMTS Bands List"))
            .style(umts_style)
            .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        let lte_bands_list = List::new(
                modem_caps.supported_lte_bands.iter().map(|b| {
                    let band_str = format!("B{}", b);
                    let checkbox = if enabled_lte_bands.contains(&b) {
                        "[x]"
                    } else {
                        "[ ]"
                    };
                    ListItem::new(format!("{} {}", checkbox, band_str))
                }).collect::<Vec<_>>()
            )
            .block(Block::bordered().borders(Borders::ALL).title("LTE Bands List"))
            .style(lte_style)
            .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        let command_to_execute = Paragraph::new(save_bands_command(self.bands_config.umts.clone(), self.bands_config.lte.clone()))
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("AT command").borders(Borders::ALL));


        frame.render_stateful_widget(umts_bands_list, bands_lists_area[0], &mut umts_bands_list_state);
        frame.render_stateful_widget(lte_bands_list, bands_lists_area[1], &mut lte_bands_list_state);
        frame.render_widget(command_to_execute, tab_area[1]);
    }
}
