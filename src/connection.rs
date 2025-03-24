use toydb::{client, error::Result};

pub struct Connection {
    client: Option<client::Client>,
}

impl Connection {
    pub fn new() -> Self {
        Connection { client: None }
    }

    /// connects to the toydb server
    pub fn connect(&self, host: &str, port: u16) -> Result<Connection> {
        let connection = client::Client::connect((host, port))?;

        Ok(Connection {
            client: Some(connection),
        })
    }
}
