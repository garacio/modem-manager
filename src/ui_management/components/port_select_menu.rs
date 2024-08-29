use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::Frame;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use serialport::SerialPortInfo;
use crate::serial_tools::serial_reader::list_ports;
use crate::ui_management::components::ComponentRender;

#[derive(Debug)]
pub struct PortSelectMenu {
    pub(crate) ports: Vec<String>,
    pub(crate) ports_info: Vec<SerialPortInfo>,
    pub(crate) list_state: ListState,
    selected_port: Option<String>,
}

impl PortSelectMenu {
    pub fn new() -> Self {
        let ports_info = list_ports().unwrap_or_else(|_| vec![]);
        let ports = ports_info.clone()
            .iter()
            .map(|port| port.port_name.clone())
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            ports,
            ports_info,
            list_state,
            selected_port: None,
        }
    }

    // pub fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), std::io::Error> {
    //     terminal.draw(|f| {
    //         let size = f.area();
    //
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .constraints(
    //                 [
    //                     Constraint::Percentage(90),
    //                     Constraint::Percentage(10),
    //                 ]
    //                 .as_ref(),
    //             )
    //             .split(size);
    //
    //         let items: Vec<ListItem> = self.ports
    //             .iter()
    //             .map(|port| ListItem::new(port.clone()))
    //             .collect();
    //
    //         let ports_list = List::new(items)
    //             .block(Block::default().title("Select Port").borders(Borders::ALL))
    //             .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black).add_modifier(Modifier::BOLD))
    //             .highlight_symbol("> ");
    //
    //         f.render_stateful_widget(ports_list, chunks[0], &mut self.list_state);
    //
    //         let instructions = Paragraph::new("Use Up/Down arrows to select a port, Enter to confirm, Q to quit.")
    //             .style(Style::default().fg(Color::White).bg(Color::Black));
    //
    //         f.render_widget(instructions, chunks[1]);
    //     })?;
    //
    //     Ok(())
    // }

    pub fn handle_event(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Down => {
                    let i = match self.list_state.selected() {
                        Some(i) => {
                            if i >= self.ports.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.list_state.select(Some(i));
                }
                KeyCode::Up => {
                    let i = match self.list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                self.ports.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.list_state.select(Some(i));
                }
                _ => {}
            }
        }
    }

    pub fn selected_port(&self) -> Option<String> {
        self.selected_port.clone()
    }
}

pub struct RenderProps {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

impl ComponentRender<RenderProps> for PortSelectMenu {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let items: Vec<ListItem> = self.ports
            .iter()
            .map(|port| ListItem::new(port.clone()))
            .collect();
        let ports_list = List::new(items)
            .block(Block::default().title(props.title).borders(Borders::ALL))
            .highlight_style(Style::default().bg(props.border_color).fg(Color::Black).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        let mut list_state = self.list_state.clone();

        #[cfg(not(debug_assertions))]
        {
            frame.render_stateful_widget(ports_list, props.area, &mut list_state);
        }

        #[cfg(debug_assertions)]
        {
            let content_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(props.area);

            frame.render_stateful_widget(ports_list, content_area[0], &mut list_state);

            let debug_line = Paragraph::new(format!("{self:#?}"));
            frame.render_widget(debug_line, content_area[1]);
        }
    }
}