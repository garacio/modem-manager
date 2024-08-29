use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::ui_management::components::main_with_tabs::MainWithTabs;

impl MainWithTabs {

    pub(crate) fn byte_index(&self) -> usize {
        self.terminal_user_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.terminal_user_input.len())
    }

    pub(crate) fn enter_char(&mut self, ch: char) {
        let index = self.byte_index();
        self.terminal_user_input.insert(index, ch);
        self.move_cursor_right();
    }

    pub(crate) fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.terminal_user_input.chars().count())
    }

    pub(crate) fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
        self.cursor_position.x = self.cursor_index as u16 + 1;
    }

    pub(crate) fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
        self.cursor_position.x = self.cursor_index as u16 + 1;
    }

    pub(crate) fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.terminal_user_input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.terminal_user_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.terminal_user_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn render_terminal_tab(&mut self, frame: &mut Frame, area: Rect) {
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
AT+XACT=4,2,,0 - UMTS+LTE all bands, LTE preferred
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

        let input = Paragraph::new(self.terminal_user_input.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().title("Input").borders(Borders::ALL));

        let output = Paragraph::new(self.terminal_user_output.clone().join("\n"))
            .style(Style::default().fg(Color::White))
            .block(Block::default().title("Output").borders(Borders::ALL));

        frame.render_widget(help_note, h_chunks[1]);
        frame.render_widget(output, v_chunks[0]);
        frame.render_widget(input, v_chunks[1]);

        frame.set_cursor_position(Position::from((self.cursor_position.x, v_chunks[1].y+1)));
    }
}