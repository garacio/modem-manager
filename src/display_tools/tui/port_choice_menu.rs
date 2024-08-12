use std::io::Stdout;
use std::time::Duration;
use crossterm::event;
use crossterm::event::Event::Key;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use crate::serial_tools::serial_reader::list_ports;

pub fn show_port_choice_menu(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<String, std::io::Error> {
    let ports = list_ports().unwrap();

    let port_names: Vec<String> = ports.iter().map(|port| port.port_name.clone()).collect();

    // let port_names = list_ports().unwrap().iter().map(|port| port.port_name );
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(90),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(size);

            let items: Vec<ListItem> = port_names
                .iter()
                .map(|port| ListItem::new(port.clone()))
                .collect();
            let ports_list = List::new(items)
                .block(Block::default().title("Select Port").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black).add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(ports_list, chunks[0], &mut list_state);

            let instructions = Paragraph::new("Use Up/Down arrows to select a port, Enter to confirm, Q to quit.")
                .style(Style::default().fg(Color::White).bg(Color::Black));
            f.render_widget(instructions, chunks[1]);
        }).ok();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Ok(Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok("".to_string()),
                        KeyCode::Down => {
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i >= port_names.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            list_state.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        port_names.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            list_state.select(Some(i));
                        }
                        KeyCode::Enter => {
                            if let Some(i) = list_state.selected() {
                                return Ok(port_names[i].clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}