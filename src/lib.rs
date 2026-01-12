//main file to write the value
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfo, ReplicaTransactionInfo,
};
use solana_logger::setup;
use std::sync::{Arc, Mutex};
mod config;

use config::{Config, makeconfig};

// the main brain struct
#[derive(Debug)]
pub struct DataStruct {
    //figure the issue
    database_url: String,
    tempstorage: String,
    last_successful_update_time: u8,
    error_count: u8,
    total_transaction_processed: u8,
}

#[derive(Debug)]
pub struct MainState {
    state: Arc<Mutex<DataStruct>>,
}
impl GeyserPlugin for MainState {
    //gets the name for the plugin
    fn name(&self) -> &'static str {
        let name = "txlogger";
        name
    }

    // the main onload method
    fn on_load(
        &mut self,
        _config_file: &str,
        _is_reload: bool,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        //call the function to create struct from config.rs
        let data = makeconfig(_config_file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        //add the database_url to the main state struct
        self.state.lock().unwrap().database_url = data.database_url;
        Ok(())
    }
}
