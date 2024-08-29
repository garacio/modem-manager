use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use crate::modem_tools::modem::modem_execute;
use crate::terminator::{Interrupted, Terminator};
use super::state::State;
use super::action::Action;
use crate::modem_tools::ModemInfoUpdater;

pub struct StateStore {
    state_tx: UnboundedSender<State>,
}

impl StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State>();
        (StateStore{state_tx}, state_rx)
    }

    pub async fn main_loop(self, mut terminator: Terminator, mut action_rx: UnboundedReceiver<Action>) -> anyhow::Result<Interrupted> {
        let mut state = State::default();
        self.state_tx.send(state.clone())?;

        let mut ticker = tokio::time::interval(Duration::from_millis(250));

        loop {
            tokio::select! {
                 Some(action) = action_rx.recv() => {
                    match action {
                        Action::SetPortName(port_name) => {
                            state.port_name = port_name;
                            let mut modem_info_updater = ModemInfoUpdater::new(state.clone(), self.state_tx.clone());
                            tokio::spawn( async move {
                                modem_info_updater.read_modem_data().await;
                                });
                            self.state_tx.send(state.clone())?;
                        }
                        Action::UpdateStateFromModem(modem_info) => {
                            state.modem_info = modem_info;
                            self.state_tx.send(state.clone())?
                        }
                        Action::ReadModemData(_) => {

                        }
                        Action::ExecuteModemCommand(command) => {
                            if let terminal_output = modem_execute(&state.port_name, &state.baud_rate, &command)? {
                                state.terminal_user_output = terminal_output;
                                self.state_tx.send(state.clone())?;
                            }
                        }
                        Action::Exit => {
                            let _ = terminator.terminate(Interrupted::UserInt);
                            break Ok(Interrupted::UserInt);
                        }
                    }

                    // self.state_tx.send(state.clone())?;
                }

                _ = ticker.tick() => { }
            }
        }
    }
}