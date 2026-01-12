//main file to write the value
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfo, ReplicaTransactionInfo,
    ReplicaTransactionInfoV2, ReplicaTransactionInfoVersions,
};
use solana_program::pubkey::Pubkey;
use std::sync::{Arc, Mutex};
mod config;
mod database;
mod plugin;
use config::makeconfig;
use std::fs::{File, OpenOptions};
use std::io::Write;

const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
// the main brain struct
#[derive(Debug)]
pub struct DataStruct {
    //figure the issue
    database_url: String,
    tempstorage: String,
    last_successful_update_time: u8,
    error_count: u8,
    total_transaction_processed: u8,
    txlog_file: Option<File>,
}

use solana_program::clock::Slot;

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

        // 1. Open the file ONCE
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("whale_alerts.txt")
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        // 2. Save it into your state
        let mut state = self.state.lock().unwrap();
        state.txlog_file = Some(file);

        Ok(())
    }

    //the main notify_transaction method
    fn notify_transaction(
        &self,
        transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let data = match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(info) => {}
            ReplicaTransactionInfoVersions::V0_0_2(info) => {
                self::MainState::process_manager(self, &info);
            }
        };

        Ok(())
    }
}

impl MainState {
    pub fn process_manager(&self, info: &ReplicaTransactionInfoV2) {
        let meta = info.transaction_status_meta;
        if let Some(post_balance) = &meta.post_token_balances {
            //loop the post balance
            for balance in post_balance {
                if balance.mint == USDC_MINT {
                    //calculate the balance difference

                    let pre_balance_amount = meta
                        .pre_token_balances
                        .as_ref()
                        .and_then(|pres| {
                            pres.iter()
                                .find(|p| p.account_index == balance.account_index)
                        })
                        .map(|p| p.ui_token_amount.amount.parse::<u64>().unwrap_or(0));

                    let post_amount = balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);

                    //comapre the amounts
                    if post_amount > pre_balance_amount.unwrap() {
                        let diff = post_amount - pre_balance_amount.unwrap();

                        if diff > 1_000_000_000 {
                            let accounts_keys = info.transaction.message().account_keys();
                            let owner_address =
                                accounts_keys.get(balance.account_index as usize).unwrap();

                            //append data to a txt file -- for now --
                            let message = format!(
                                "WHALE ALERT | Slot: {} | Tx: {:?} | User: {:?} | Amount: {} USDC\n",
                                0, // You can pass 'slot' to this function if you want precise slot numbers
                                info.signature,
                                owner_address,
                                diff / 1_000_000
                            );

                            let mut state = self.state.lock().unwrap();
                            if let Some(file) = &mut state.txlog_file {
                                let _ = file.write_all(message.as_bytes());
                            }

                            println!(
                                "ALERT! USER : {:?} |  RECEIVED: {} USDC",
                                owner_address,
                                diff / 1_000_000
                            )
                        }
                    }
                }
            }
        }
    }
}
