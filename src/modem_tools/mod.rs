use tokio::time::sleep;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use crate::modem_tools::modem::{get_modem_info, get_modem_info_string};
use crate::state_store::state::State;

pub mod converters;
pub mod modem;
pub mod supported_modems;

pub(crate) struct ModemInfoUpdater {
    state_tx: UnboundedSender<State>,
    state: State
}

impl ModemInfoUpdater {
    pub fn new(state: State, state_tx: UnboundedSender<State>) -> Self {
        Self {
            state,
            state_tx
        }
    }

    pub async fn read_modem_data(&mut self) {
        loop {
            let baud_rate = 115_200;
            let modem_info_string = get_modem_info_string(self.state.port_name.clone().as_str(), baud_rate).unwrap_or_else(|err| {
                eprintln!("{}", err);
                String::new()
            });

            if let Ok(modem_info) = get_modem_info(modem_info_string).await {
                self.state.modem_info = modem_info.clone();
                self.state_tx.send(self.state.clone())
            } else {
                Ok(())
            }.expect("TODO: panic message");
            sleep(Duration::from_secs(3)).await;
        }
    }
}