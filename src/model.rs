use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Clone, Tabled)]
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

impl From<sshconfig::HostEntry> for Server {
    fn from(host: sshconfig::HostEntry) -> Self {
        Self {
            name: host.name,
            host: host.host,
            port: host.port.unwrap_or(22),
            user: host.user,
            password: None,
            identity_file: host.identity_file,
            current: None,
        }
    }
}

impl Server {
    /// 创建一个新的Config实例
    pub fn new(host: String) -> Self {
        Self {
            host: host.clone(),
            name: host,
            port: 22,
            user: "root".to_string(),
            password: None,
            identity_file: "~/.ssh/id_rsa".to_string().into(),
            current: None,
        }
    }
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
