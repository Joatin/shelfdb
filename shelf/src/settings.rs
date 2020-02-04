use config::{Config, ConfigError};
use std::collections::HashMap;
use failure::Error;
use slog::Logger;
use colored::*;
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub data_folder: String,
    pub client_server_port: u16,
    pub admin_server_port: u16,
    pub host: String
}

impl Settings {
    pub fn load(logger: &Logger) -> Result<Self, Error> {
        let mut settings = Config::new();

        Settings::set_defaults(&mut settings)?;

        info!(logger, "Loading configurations");
        if let Err(_) = settings.merge(config::File::with_name("shelf")) {
            warn!(logger, "No config file found, you can add a shelf.yml, shelf.json or shelf.toml file. Using defaults for now...")
        }

        settings.merge(config::Environment::with_prefix("SHELF")).unwrap();

        let res: Settings = settings.try_into()?;

        info!(logger, "Using data folder {}", format!("\"{}\"", &res.data_folder).yellow());
        trace!(logger, "Entire configuration object was: {:?}", &res);

        if let Err(e) = res.admin_host() {
            crit!(logger, "The admin host was invalid! You need to provide a valid host, please correct the host and try again. Incorrect key was {} or {}", " host ".on_red(), " clientServerPort ".on_red(); "host" => res.host, "port" => res.client_server_port, "error" => format!("{}", e));
            return Err(e);
        }
        if let Err(e) = res.client_host() {
            crit!(logger, "The client host was invalid! You need to provide a valid host, please correct the host and try again. Incorrect key was {} or {}", " host ".on_red(), " adminServerPort ".on_red(); "host" => res.host, "port" => res.admin_server_port, "error" => format!("{}", e));
            return Err(e);
        }

        Ok(res)
    }

    fn set_defaults(settings: &mut Config) -> Result<(), Error> {
        settings.set_default("dataFolder", ".shelf_data")?;
        settings.set_default("clientServerPort", 5600)?;
        settings.set_default("adminServerPort", 5601)?;
        settings.set_default("host", "127.0.0.1")?;

        Ok(())
    }

    pub fn client_host(&self) -> Result<SocketAddr, Error> {
        let mut addr: SocketAddr = format!("{}:{}", self.host, self.client_server_port).parse()?;
        Ok(addr)
    }

    pub fn admin_host(&self) -> Result<SocketAddr, Error> {
        let mut addr: SocketAddr = format!("{}:{}", self.host, self.admin_server_port).parse()?;
        Ok(addr)
    }
}