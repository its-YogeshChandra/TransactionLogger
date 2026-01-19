//main file to write the value
use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaTransactionInfoV2, ReplicaTransactionInfoV3,
    ReplicaTransactionInfoVersions,
};
use solana_clock::Slot;
use std::sync::{Arc, Mutex};
mod config;
use config::makeconfig;
use std::fs::{File, OpenOptions};
use std::io::Write;

const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(Debug)]
pub struct DataStruct {
    database_url: String,
    error_count: u8,
    txlog_file: Option<File>,
}

#[derive(Debug)]
pub struct MainState {
    state: Arc<Mutex<DataStruct>>,
}

impl GeyserPlugin for MainState {
    fn name(&self) -> &'static str {
        "txlogger"
    }

    fn on_load(
        &mut self,
        _config_file: &str,
        _is_reload: bool,
    ) -> agave_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let data = makeconfig(_config_file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        self.state.lock().unwrap().database_url = data.database_url;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("whale_alerts.txt")
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;

        let mut state = self.state.lock().unwrap();
        state.txlog_file = Some(file);

        eprintln!("🔔 PLUGIN: on_load complete!");
        Ok(())
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    fn notify_transaction(
        &self,
        transaction: agave_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: Slot,
    ) -> agave_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        // DEBUG: Log EVERY transaction that comes in
        eprintln!("📥 PLUGIN: notify_transaction called at slot {}", slot);
        
        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(_) => {
                eprintln!("  └─ Version: V0_0_1 (skipped)");
            }
            ReplicaTransactionInfoVersions::V0_0_2(info) => {
                eprintln!("  └─ Version: V0_0_2, sig: {:?}", info.signature);
                self.process_manager_v2(&info, slot);
            }
            ReplicaTransactionInfoVersions::V0_0_3(info) => {
                eprintln!("  └─ Version: V0_0_3, sig: {:?}", info.signature);
                self.process_manager_v3(&info, slot);
            }
        };
        Ok(())
    }
}

impl MainState {
    fn process_manager_v2(&self, info: &ReplicaTransactionInfoV2, slot: Slot) {
        let meta = info.transaction_status_meta;
        
        // DEBUG: Check if we have token balances
        eprintln!("    📊 V2 Processing - has post_token_balances: {}", meta.post_token_balances.is_some());
        
        if let Some(post_balance) = &meta.post_token_balances {
            eprintln!("    📊 Found {} post_token_balances", post_balance.len());
            
            for balance in post_balance {
                eprintln!("      └─ Token balance entry, account_index: {}", balance.account_index);
                
                // Track all tokens for testing
                let pre_balance_amount = meta
                    .pre_token_balances
                    .as_ref()
                    .and_then(|pres| {
                        pres.iter()
                            .find(|p| p.account_index == balance.account_index)
                    })
                    .map(|p| p.ui_token_amount.amount.parse::<u64>().unwrap_or(0));

                let post_amount = balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                
                eprintln!("      └─ Pre: {:?}, Post: {}", pre_balance_amount, post_amount);

                if let Some(pre_amount) = pre_balance_amount {
                    if post_amount != pre_amount {
                        let diff = if post_amount > pre_amount {
                            post_amount - pre_amount
                        } else {
                            pre_amount - post_amount
                        };
                        
                        // Log ALL balance changes for debugging (threshold = 0)
                        let account_keys = info.transaction.message().account_keys();
                        if let Some(owner) = account_keys.get(balance.account_index as usize) {
                            let message = format!(
                                "TOKEN CHANGE | Slot: {} | Tx: {:?} | Account: {:?} | Pre: {} | Post: {} | Diff: {}\n",
                                slot, info.signature, owner, pre_amount, post_amount, diff
                            );
                            eprintln!("� {}", message);
                            let mut state = self.state.lock().unwrap();
                            if let Some(file) = &mut state.txlog_file {
                                let _ = file.write_all(message.as_bytes());
                            }
                        }
                    }
                }
            }
        }
    }

    fn process_manager_v3(&self, info: &ReplicaTransactionInfoV3, slot: Slot) {
        let meta = info.transaction_status_meta;
        
        // DEBUG: Check if we have token balances
        eprintln!("    📊 V3 Processing - has post_token_balances: {}", meta.post_token_balances.is_some());
        
        if let Some(post_balance) = &meta.post_token_balances {
            eprintln!("    📊 Found {} post_token_balances", post_balance.len());
            
            for balance in post_balance {
                eprintln!("      └─ Token balance entry, account_index: {}", balance.account_index);
                
                let pre_balance_amount = meta
                    .pre_token_balances
                    .as_ref()
                    .and_then(|pres| {
                        pres.iter()
                            .find(|p| p.account_index == balance.account_index)
                    })
                    .map(|p| p.ui_token_amount.amount.parse::<u64>().unwrap_or(0));

                let post_amount = balance.ui_token_amount.amount.parse::<u64>().unwrap_or(0);
                
                eprintln!("      └─ Pre: {:?}, Post: {}", pre_balance_amount, post_amount);

                if let Some(pre_amount) = pre_balance_amount {
                    if post_amount != pre_amount {
                        let diff = if post_amount > pre_amount {
                            post_amount - pre_amount
                        } else {
                            pre_amount - post_amount
                        };
                        
                        let account_keys = info.transaction.message.static_account_keys();
                        if let Some(owner) = account_keys.get(balance.account_index as usize) {
                            let message = format!(
                                "TOKEN CHANGE | Slot: {} | Tx: {:?} | Account: {:?} | Pre: {} | Post: {} | Diff: {}\n",
                                slot, info.signature, owner, pre_amount, post_amount, diff
                            );
                            eprintln!("� {}", message);
                            let mut state = self.state.lock().unwrap();
                            if let Some(file) = &mut state.txlog_file {
                                let _ = file.write_all(message.as_bytes());
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for MainState {
    fn default() -> Self {
        MainState {
            state: Arc::new(Mutex::new(DataStruct {
                database_url: String::new(),
                error_count: 0,
                txlog_file: None,
            })),
        }
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = MainState::default();
    let boxed: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(boxed)
}
