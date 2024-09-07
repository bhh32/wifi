#[derive(Debug, Clone)]
pub enum Message {
    Add,
    Remove,
    UpdateConName(String),
    UpdateIface(String),
    UpdateSsid(String),
    UpdatePsk(String),
    UpdateIPv4(String),
    UpdateNetmask(String),
    UpdateGateway(String),
    UpdateDns(String),
    UpdateHidden(bool),
    UpdateAutoCon(bool),
    UpdateDHCP(bool),
    EnterEmptyInput(String),
}