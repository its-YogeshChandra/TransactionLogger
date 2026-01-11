mod config;
mod database;
mod plugin;

use bs58;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    ReplicaAccountInfo, ReplicaAccountInfoV2,
};
use std::path::Path;

//brain struct for the plugin
pub struct MainData {
    databaseurl: String,
    maindata: String,
    accountscounter: u8,
}

impl MainData {
    pub fn create_new(databaseurl: String, maindata: String, accountscounter: u8) -> Self {
        Self {
            databaseurl: databaseurl,
            maindata: maindata,
            accountscounter: accountscounter,
        }
    }
}

//try to look into the solana_geyser_plugin_interface
fn main() {
    println!("Hello, world!");

    //call you maybe
}
