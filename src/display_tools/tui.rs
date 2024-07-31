use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Text};
use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem, ListState};
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::process::exit;
use crossterm::event::Event::Key;
use crate::serial_tools::serial_reader::list_ports;
use crate::modem_tools::modem::{get_modem_info, get_modem_info_string};

fn initialize_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn cleanup_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn show_port_choice_menu(terminal: &mut Terminal<CrosstermBackend<Stdout>>, port_names: &[String]) -> Option<String> {
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Percentage(20),
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
        }).ok()?;

        if event::poll(Duration::from_millis(100)).ok()? {
            if let Key(key) = event::read().ok()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return None,
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
                                return Some(port_names[i].clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    port_name: &str,
    baud_rate: u32,
) -> Result<(), io::Error> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_secs(3);

    thread::spawn(move || {
        loop {
            if tx.send(()).is_err() {
                break;
            }
            thread::sleep(tick_rate);
        }
    });

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if let Ok(_) = rx.try_recv() {
            let modem_info_string = get_modem_info_string(&port_name, baud_rate).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            });

            let modem_info = get_modem_info(modem_info_string).unwrap();

            terminal.draw(|f| {
                let size = f.size();

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
                    .split(size);

                let modem_info_paragraph = Paragraph::new(Text::from(modem_info.display_modem_info()))
                    .block(Block::default().title("Modem Info").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD));

                let signal_info_paragraph = Paragraph::new(Text::from(modem_info.display_signal_info()))
                    .block(Block::default().title("Signal Info").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD));

                let carrier_info_paragraph = Paragraph::new(Text::from(modem_info.display_carrier_info()))
                    .block(Block::default().title("Carrier Info").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::BOLD));

                f.render_widget(modem_info_paragraph, chunks[0]);
                f.render_widget(signal_info_paragraph, chunks[1]);
                f.render_widget(carrier_info_paragraph, chunks[2]);
            })?;
        }
    }

    Ok(())
}

pub fn start_tui(baud_rate: u32) -> Result<(), io::Error> {
    let mut terminal = initialize_terminal()?;

    let ports = list_ports()?;
    if ports.is_empty() {
        eprintln!("There are no available ports");
        cleanup_terminal(terminal)?;
        return Ok(());
    }

    let port_names: Vec<String> = ports.iter().map(|port| port.port_name.clone()).collect();
    if let Some(selected_port) = show_port_choice_menu(&mut terminal, &port_names) {
        main_event_loop(&mut terminal, &selected_port, baud_rate)?;
    }

    cleanup_terminal(terminal)?;
    Ok(())
}
