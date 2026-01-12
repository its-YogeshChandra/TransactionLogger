//read the config from the config file of the validator
use figment::{
    Error, Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}

//make function
pub fn makeconfig(config_path: &str) -> Result<Config, Error> {
    let config: Config = Figment::new().merge(Toml::file(config_path)).extract()?;
    Ok(config)
}
