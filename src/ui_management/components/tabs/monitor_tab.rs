
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::ui_management::components::main_with_tabs::MainWithTabs;

impl MainWithTabs {
        pub fn render_monitor_tab(&mut self, frame: &mut Frame, area: Rect) {

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

        let modem_info_p =  Paragraph::new(Text::from(self.props.modem_info.display_modem_info()))
            .block(Block::default().title("Modem Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let signal_info_p =   Paragraph::new(Text::from(self.props.modem_info.display_signal_info()))
            .block(Block::default().title("Signal Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let carrier_info_p = Paragraph::new(Text::from(self.props.modem_info.display_carrier_info()))
            .block(Block::default().title("Carrier Info").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black));

        frame.render_widget(modem_info_p, chunks[0]);
        frame.render_widget(signal_info_p, chunks[1]);
        frame.render_widget(carrier_info_p, chunks[2]);
    }
}