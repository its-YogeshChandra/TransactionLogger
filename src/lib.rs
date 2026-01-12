//main file to write the value
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfo, ReplicaTransactionInfo,
};
use std::sync::{Arc, Mutex};

// the main brain struct
#[derive(Debug)]
pub struct DataStruct {
    //figure the issue
    databaseurl: String,
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
        Ok(())
    }
}
