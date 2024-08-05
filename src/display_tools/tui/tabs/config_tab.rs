use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, StatefulWidget, Style};
use ratatui::widgets::{Block, Borders, List, ListDirection};
use crate::display_tools::tui::app_tabs::AppTabs;

impl AppTabs {
    pub fn render_config_tab(self, area: Rect, buf: &mut Buffer) {
        let lte_items = vec!["B1", "B3", "B20"];
        let state = self.band_list_state.clone();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    // Constraint::Percentage(45),
                    // Constraint::Percentage(35),
                ]
                .as_ref(),
            )
            .split(area);

        let bl = List::new(lte_items)
            .block(Block::bordered().borders(Borders::ALL).title("Band List"))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
            // .render(chunks[0], buf, state);

        StatefulWidget::render(bl, chunks[0], buf, &mut state.lock().unwrap());
    }
}
