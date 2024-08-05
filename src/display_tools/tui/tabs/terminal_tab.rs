use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Offset, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget, Wrap};
use crate::display_tools::tui::app_tabs::AppTabs;

impl AppTabs {

    pub fn render_terminal_tab(self, area: Rect, buf: &mut Buffer) {
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
                ].as_ref())
            .split(area);

        let v_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(h_chunks[0]);

        let help_note_text = "\
ATI8 - Current firmware version
AT+CGSN? - IMEI
AT+GTUSBMODE=7 - Switch to MBIM mode
AT+GTUSBMODE=9 - Switch to NCM+ACM(2EP) mode - for Kinetics
AT+CFUN=15 - Restart modem
AT+XACT? - View enabled bands
AT+XACT=2,,,107,103 - Enable only band 7 and 3
AT+XACT=2,,,107 - Enable only band 7
AT+XACT=2,,,0 - Unlock all LTE bands
AT+XLEC? - View active aggregation
at@sic:freq_lock(0,3,band,1,EARFCN,PCI) - Lock carrier frequency
        ";

        let help_note = Paragraph::new(help_note_text)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .block(Block::default().title("Usefull commands").borders(Borders::ALL));

        let input = Paragraph::new(self.terminal_data.input.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().title("Input").borders(Borders::ALL));

        let output = Paragraph::new(self.terminal_data.output)
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Output").borders(Borders::ALL));

        help_note.render(h_chunks[1], buf);
        output.render(v_chunks[0], buf);
        input.render(v_chunks[1], buf);
    }
}