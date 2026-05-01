use std::env;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;

use crate::types::{Config, Connection, Endpoint, EndpointKind, Row};

fn config_dir() -> PathBuf {
    if let Ok(dir) = env::var("SQLIT_CONFIG_DIR") {
        PathBuf::from(dir)
    } else if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg).join("sqlit")
    } else {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_owned());
        PathBuf::from(home).join(".config").join("sqlit")
    }
}

pub fn load_connections() -> Result<Vec<Connection>, String> {
    let path = config_dir().join("connections.json");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(format!("cannot read {}: {e}", path.display())),
    };
    let config: Config = serde_json::from_str(&content)
        .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
    Ok(config.connections.unwrap_or_default())
}

fn connection_info(endpoint: &Endpoint) -> String {
    match &endpoint.kind {
        Some(EndpointKind::Tcp) => {
            let mut info = String::new();
            info.push_str(endpoint.host.as_deref().unwrap_or(""));
            if let Some(port) = endpoint.port.as_deref().filter(|s| !s.is_empty()) {
                info.push(':');
                info.push_str(port);
            }
            if let Some(database) = endpoint.database.as_deref().filter(|s| !s.is_empty()) {
                info.push('/');
                info.push_str(database);
            }
            info
        }
        Some(EndpointKind::File) => endpoint.path.as_deref().unwrap_or("").to_owned(),
        _ => String::new(),
    }
}

impl From<Connection> for Row {
    fn from(conn: Connection) -> Self {
        let info = conn.endpoint.as_ref().map(connection_info).unwrap_or_default();
        let user = conn
            .endpoint
            .as_ref()
            .and_then(|e| e.username.as_deref())
            .filter(|s| !s.is_empty())
            .unwrap_or("")
            .to_owned();
        Row {
            name: conn.name.unwrap_or_default(),
            db_type: conn.db_type.unwrap_or_default(),
            info,
            user,
        }
    }
}

pub fn launch(name: &str) -> i32 {
    let status = Command::new("sqlit").args(["-c", name]).status();
    match status {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("sqlitx: sqlit executable not found in PATH");
            2
        }
        Err(e) => {
            eprintln!("sqlitx: failed to launch sqlit: {e}");
            2
        }
    }
}
