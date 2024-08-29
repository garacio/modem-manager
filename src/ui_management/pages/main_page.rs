use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedSender;
use crate::state_store::action::Action;
use crate::state_store::state::State;
use crate::ui_management::components::{Component, ComponentRender};
use crate::ui_management::components::main_with_tabs::{MainWithTabs, RenderProps, SelectedTab};


struct Props {
    error_message: Option<String>,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            error_message: None,
        }
    }
}

pub(crate) struct MainPage {
    /// Action sender
    pub action_tx: UnboundedSender<Action>,
    // Mapped Props from State
    props: Props,
    // Internal Components
    main_with_tabs: MainWithTabs,
    // pub(crate) selected_tab: SelectedTab,
}

impl Component for MainPage {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized
    {
        let main_with_tabs = MainWithTabs::new(&state);
        MainPage {
            action_tx: action_tx.clone(),
            props: Props::from(state),
            main_with_tabs
            // selected_port: state.port_name.clone(),
            // selected_tab: SelectedTab::MonitorTab,
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized
    {
        MainPage {
            props: Props::from(state),
            main_with_tabs: self.main_with_tabs.move_with_state(&state),
            ..self
        }
    }

    fn name(&self) -> &str {
        "Modem Manager"
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => {
                if self.main_with_tabs.editing_mode {
                    self.main_with_tabs.editing_mode = false
                } else {
                    let _ = self.action_tx.send(Action::Exit);
                }
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let _ = self.action_tx.send(Action::Exit);
            }
            KeyCode::Enter => {
                match self.main_with_tabs.selected_tab {
                    SelectedTab::TerminalTab => if self.main_with_tabs.editing_mode {
                        let _ = self.action_tx.send(Action::ExecuteModemCommand(self.main_with_tabs.terminal_user_input.clone()));
                        self.main_with_tabs.terminal_user_input = "".to_string();
                        self.main_with_tabs.cursor_position.x = 1;
                    },
                    _ => {}
                }
            }
            _ => self.main_with_tabs.handle_event(key)
        }
    }
}

impl ComponentRender<()> for MainPage {
     fn render(&mut self, frame: &mut Frame, _props: ()) {
        let [container_inner, container_error_message] =
            *Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(95),
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(frame.area())
        else {
            panic!("The left layout should have 2 chunks")
        };

         self.main_with_tabs.render(
             frame,
             RenderProps {
                 area: container_inner,
                 show_cursor: false,
             }

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