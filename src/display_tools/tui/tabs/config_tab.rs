use std::collections::HashSet;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style};
use ratatui::widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph, Widget};
use crate::display_tools::tui::app_tabs::AppTabs;

impl AppTabs {
    pub fn render_config_tab(self, area: Rect, buf: &mut Buffer) {
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

        let enabled_lte_bands: HashSet<_> =  self.config_lte_bands.iter().collect();
        let enabled_umts_bands: HashSet<_> = self.config_umts_bands.iter().collect();

        let modem_caps = self.modem_capabilities.spec.unwrap();

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
            .style(Style::default().fg(Color::White).bg(Color::Black))
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
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        let command_to_execute = Paragraph::new(self.save_bands_command.clone())
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("AT command").borders(Borders::ALL));

        StatefulWidget::render(umts_bands_list, bands_lists_area[0], buf, &mut umts_bands_list_state);
        StatefulWidget::render(lte_bands_list, bands_lists_area[1], buf, &mut lte_bands_list_state);
        Widget::render(command_to_execute, tab_area[1], buf);
    }
}
