use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub connections: Option<Vec<Connection>>,
}

#[derive(Deserialize)]
pub struct Connection {
    pub name: Option<String>,
    pub db_type: Option<String>,
    pub endpoint: Option<Endpoint>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointKind {
    Tcp,
    File,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize)]
pub struct Endpoint {
    pub kind: Option<EndpointKind>,
    pub username: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub database: Option<String>,
    pub path: Option<String>,
}

pub struct Row {
    pub name: String,
    pub db_type: String,
    pub info: String,
    pub user: String,
}
