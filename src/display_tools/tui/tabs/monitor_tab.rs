use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use crate::display_tools::tui::app_tabs::AppTabs;

impl AppTabs {
        pub fn render_monitor_tab(self, area: Rect, buf: &mut Buffer) {

         let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(45),
                    Constraint::Percentage(35),
                ]
                .as_ref(),
            )
            .split(area);

        let info = self.modem_info.lock().unwrap();

        Paragraph::new(Text::from(info.display_modem_info()))
            .block(Block::default().title("Modem Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black)).render(chunks[0], buf);

        Paragraph::new(Text::from(info.display_signal_info()))
            .block(Block::default().title("Signal Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black)).render(chunks[1], buf);

        Paragraph::new(Text::from(info.display_carrier_info()))
            .block(Block::default().title("Carrier Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black)).render(chunks[2], buf);
    }
}