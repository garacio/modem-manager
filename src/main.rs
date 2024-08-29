mod serial_tools;
mod modem_tools;
// mod display_tools;
mod tests;
mod state_store;
mod ui_management;
mod terminator;

use state_store::state_store::StateStore;
use ui_management::ui_manager::UiManager;
use crate::terminator::{create_termination, Interrupted};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // run_app()?;
    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new();
    let (ui_manager, action_rx) = UiManager::new();

    tokio::try_join!(
        state_store.main_loop(terminator, action_rx),
        ui_manager.main_loop(state_rx, interrupt_rx.resubscribe())
    )?;

    if let Ok(reason) = interrupt_rx.recv().await {
        match reason {
            Interrupted::UserInt => println!("exited per user request"),
            Interrupted::OsSigInt => println!("exited because of an os sig int"),
        }
    } else {
        println!("exited because of an unexpected error");
    }


    Ok(())
}
