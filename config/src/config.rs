use colored::*;
use config::Config as CConfig;
use failure::Error;
use slog::Logger;
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_folder: String,
    pub client_server_port: u16,
    pub admin_server_port: u16,
    pub host: String,
    pub log_level: String,
}

/// The configuration object for the Shelf database. This struct holds all configuration
///properties, and takes of loading them from the appropriate source
impl Config {
    /// Loads the config, this both checks if there is a provided config file or if there are any environment variables providing configuration
    pub fn load(logger: &Logger) -> Result<Self, Error> {
        let mut config = CConfig::new();

        Config::set_defaults(&mut config)?;

        info!(logger, "Loading configurations");
        if config.merge(config::File::with_name("shelf")).is_err() {
            warn!(logger, "No config file found, you can add a shelf.yml, shelf.json or shelf.toml file. Using defaults for now...")
        }

        config
            .merge(config::Environment::with_prefix("SHELF"))
            .unwrap();

        let res: Config = config.try_into()?;

        info!(
            logger,
            "Using data folder {}",
            format!("\"{}\"", &res.data_folder).yellow()
        );
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

    fn set_defaults(config: &mut CConfig) -> Result<(), Error> {
        config.set_default("dataFolder", ".shelf_data")?;
        config.set_default("clientServerPort", 5600)?;
        config.set_default("adminServerPort", 5601)?;
        config.set_default("host", "127.0.0.1")?;
        config.set_default("logLevel", "info")?;

        Ok(())
    }

    /// Gets the provided expected host name for the client graphql endpoint
    ///
    /// # Example
    ///
    /// ```
    /// use shelf_config::Config;
    ///
    /// let config = Config::default();
    ///
    /// let host = config.client_host();
    /// ```
    pub fn client_host(&self) -> Result<SocketAddr, Error> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.client_server_port).parse()?;
        Ok(addr)
    }

    /// Gets the provided expected host name for the admin graphql endpoint
    ///
    /// # Example
    ///
    /// ```
    /// use shelf_config::Config;
    ///
    /// let config = Config::default();
    ///
    /// let host = config.admin_host();
    /// ```
    pub fn admin_host(&self) -> Result<SocketAddr, Error> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.admin_server_port).parse()?;
        Ok(addr)
    }
}
