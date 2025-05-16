use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Clone, Tabled,PartialEq,Eq)]
#[tabled(rename_all = "UPPERCASE")]
pub(crate) struct Server {
    pub(crate) name: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display = "display_password")]
    pub(crate) password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display("tabled::derive::display::option", ""))]
    #[tabled(rename = "IDENTITY FILE")]
    pub(crate) identity_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(display = "display_option_bool")]
    #[tabled(rename = "")]
    #[tabled(order = 0)]
    pub(crate) current: Option<bool>,
}

pub(crate) fn display_password(value: &Option<String>) -> String {
    value.as_ref().map_or("", |_| "******").to_string()
}

pub(crate) fn display_option_bool(value: &Option<bool>) -> String {
    value.map_or(" ", |v| if v { "✲" } else { " " }).to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) servers: Vec<Server>,
}
