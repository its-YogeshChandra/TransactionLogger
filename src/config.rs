//read the config from the config file of the validator
use figment::{
    Error, Figment,
    providers::{Format, Json, Toml},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}

pub fn makeconfig(config_path: &str) -> Result<Config, Error> {
    let config: Config = if config_path.ends_with(".json") {
        Figment::new().merge(Json::file(config_path)).extract()?
    } else {
        Figment::new().merge(Toml::file(config_path)).extract()?
    };
    Ok(config)
}
