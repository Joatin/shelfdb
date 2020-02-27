use colored::*;
use config::Config as CConfig;
use failure::Error;
use slog::Logger;
use std::{
    net::SocketAddr,
    time::Duration,
};

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_folder: String,
    pub port: u16,
    pub host: String,
    pub log_level: String,
    #[serde(with = "serde_humanize_rs")]
    pub save_interval: Duration,
}

/// The configuration object for the Shelf database. This struct holds all
/// configuration properties, and takes of loading them from the appropriate
/// source
impl Config {
    /// Loads the config, this both checks if there is a provided config file or
    /// if there are any environment variables providing configuration
    pub fn load(logger: &Logger) -> Result<Self, Error> {
        let mut config = CConfig::new();

        Config::set_defaults(&mut config)?;

        info!(logger, "Loading configurations");
        if config
            .merge(config::File::with_name("shelf").required(false))
            .is_err()
        {
            info!(logger, "🌟 TIP: No config file found, you can add a shelf.yml, shelf.json or shelf.toml file if you wish");
        }

        if let Err(err) = config.merge(config::Environment::with_prefix("SHELF")) {
            trace!(logger, "No env vars found"; "error" => format!("{}", err));
        };

        let res: Config = config.try_into()?;

        info!(
            logger,
            "Using data folder {}",
            format!("\"{}\"", &res.data_folder).yellow()
        );
        trace!(logger, "Entire configuration object was: {:?}", &res);

        if let Err(e) = res.host() {
            crit!(logger, "The client host was invalid! You need to provide a valid host, please correct the host and try again. Incorrect key was {} or {}", " host ".on_red(), " port ".on_red(); "host" => res.host, "port" => res.port, "error" => format!("{}", e));
            return Err(e);
        }

        Ok(res)
    }

    fn set_defaults(config: &mut CConfig) -> Result<(), Error> {
        config.set_default("dataFolder", ".shelf_data")?;
        config.set_default("port", 5600)?;
        config.set_default("host", "127.0.0.1")?;
        config.set_default("logLevel", "info")?;
        config.set_default("saveInterval", "30s")?;

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
    /// let host = config.host();
    /// ```
    pub fn host(&self) -> Result<SocketAddr, Error> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.port).parse()?;
        Ok(addr)
    }
}
