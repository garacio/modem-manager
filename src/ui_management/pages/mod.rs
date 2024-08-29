mod port_select_page;
mod main_page;

use crossterm::event::KeyEvent;
use ratatui::Frame;
use strum_macros::{Display, EnumCount, EnumIter, FromRepr};
use tokio::sync::mpsc::UnboundedSender;
use crate::state_store::action::Action;
use crate::state_store::state::State;
use crate::ui_management::pages::main_page::MainPage;
use super::components::{Component, ComponentRender};
use super::pages::port_select_page::PortSelectMenuPage;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, EnumCount)]
pub enum Tabs {
    #[default]
    #[strum(to_string = "Signal monitor")]
    MonitorTab,
    #[strum(to_string = "Select bands")]
    BandsConfigTab,
    #[strum(to_string = "Terminal")]
    TerminalTab,
}
enum ActivePage {
    PortSelectMenu,
    MainPage,
}

struct Props {
    active_page: ActivePage,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            active_page: match &state.port_name {
                s if s.is_empty() => ActivePage::PortSelectMenu,
                _ => ActivePage::MainPage,
            },
        }
    }
}

pub struct AppRouter {
    props: Props,
    //
    port_select_menu: PortSelectMenuPage,
    main_page: MainPage,
}

impl AppRouter {
    fn get_active_page_component(&self) -> &dyn Component {
        match self.props.active_page {
            ActivePage::PortSelectMenu => &self.port_select_menu,
            ActivePage::MainPage => &self.main_page
        }
    }

    fn get_active_page_component_mut(&mut self) -> &mut dyn Component {
        match self.props.active_page {
            ActivePage::PortSelectMenu => &mut self.port_select_menu,
            ActivePage::MainPage => &mut self.main_page
        }
    }
}

impl Component for AppRouter {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized
    {
        AppRouter {
            props: Props::from(state),
            port_select_menu: PortSelectMenuPage::new(state, action_tx.clone()),
            main_page: MainPage::new(state, action_tx.clone()),
        }
    }

    fn move_with_state(self, state: &State) -> Self
    where
        Self: Sized
    {
        AppRouter {
            props: Props::from(state),
            port_select_menu: self.port_select_menu.move_with_state(state),
            main_page: self.main_page.move_with_state(state),
        }
    }

    fn name(&self) -> &str {
        self.get_active_page_component().name()
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        self.get_active_page_component_mut().handle_key_event(key)
    }
}

impl ComponentRender<()> for AppRouter {
    fn render(&mut self, frame: &mut Frame, props: ()) {
        match self.props.active_page {
            ActivePage::PortSelectMenu => self.port_select_menu.render(frame, props),
            ActivePage::MainPage => self.main_page.render(frame, props)
        }
    }
}
