use super::state::ModemInfo;

pub enum Action {
    SetPortName(String),
    UpdateStateFromModem(ModemInfo),
    ReadModemData(String),
    ExecuteModemCommand(String),
    Exit,
}