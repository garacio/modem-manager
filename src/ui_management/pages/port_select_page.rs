use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::style::Color;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedSender;
use crate::state_store::action::Action;
use crate::state_store::state::{State, ModemConnectionStatus};
use crate::ui_management::components::{port_select_menu, Component, ComponentRender};
use crate::ui_management::components::port_select_menu::PortSelectMenu;

struct Props {
    error_message: Option<String>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            error_message: if let ModemConnectionStatus::Errored { err } = &state.modem_connection_status {
                Some(err.to_string())
            } else { None },
        }
    }
}

pub struct PortSelectMenuPage {
    /// Action sender
    pub action_tx: UnboundedSender<Action>,
    // Mapped Props from State
    props: Props,
    // Internal Components
    port_select_menu: PortSelectMenu,
}

impl Component for PortSelectMenuPage {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized
    {
        let port_select_menu = PortSelectMenu::new();
        PortSelectMenuPage {
            action_tx: action_tx.clone(),
            props: Props::from(state),
            port_select_menu,
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized
    {
        PortSelectMenuPage {
            props: Props::from(state),
            ..self
        }
    }

    fn name(&self) -> &str {
        "Port Select Menu"
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                let _ = self.action_tx.send(Action::Exit);
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = self.action_tx.send(Action::Exit);
            }
            KeyCode::Enter => {
                if let Some(i) = self.port_select_menu.list_state.selected() {
                    let _ = self.action_tx.send(Action::SetPortName(self.port_select_menu.ports[i].clone()));
                }
            }
            _ => self.port_select_menu.handle_event(key)
        }
    }
}

impl ComponentRender<()> for PortSelectMenuPage {
    fn render(&mut self, frame: &mut Frame, _props: ()) {
        let [container_addr_input, container_error_message] =
            *Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(90),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(frame.area())
        else {
            panic!("The left layout should have 2 chunks")
        };

        self.port_select_menu.render(
            frame,
            port_select_menu::RenderProps {
                title: "Select COM port to connect".into(),
                area: container_addr_input,
                border_color: Color::Gray,
                show_cursor: true,
            },
        );

        let error_message = Paragraph::new(if let Some(err) = self.props.error_message.as_ref() {
            Text::from(format!("Error: {}", err.as_str()))
        } else {
            Text::from("")
        })
        .wrap(Wrap { trim: true })
        .style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC),
        );

        frame.render_widget(error_message, container_error_message);
    }
}