/// Module responsible for notifying plugins about entries
use {
    crate::{
        // entry_notifier_interface::EntryNotifier,
            geyser_plugin_manager::GeyserPluginManager},
    log::*,
    solana_entry::entry::EntrySummary,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        ReplicaEntryInfoV2, ReplicaEntryInfoVersions,
    },
    solana_ledger::entry_notifier_interface::EntryNotifier,
    solana_measure::measure::Measure,
    solana_metrics::*,
    solana_sdk::clock::Slot,
    solana_entry::entry::UntrustedEntry,
    std::sync::{Arc, RwLock},
};

pub(crate) struct EntryNotifierImpl {
    plugin_manager: Arc<RwLock<GeyserPluginManager>>,
}

impl EntryNotifier for EntryNotifierImpl {
    /// Notify the entry
    // fn notify_entry(&self, entry: &UntrustedEntry) {
        fn notify_entry<'a>(
            &'a self,
            slot: Slot,
            index: usize,
            entry: &'a EntrySummary,
            starting_transaction_index: usize,
        ) {
        let mut measure = Measure::start("geyser-plugin-notify_plugins_of_entry_info");
        let mut plugin_manager = self.plugin_manager.write().unwrap();
        if plugin_manager.plugins.is_empty() {
            return;
        }

        let entry_info =
            Self::build_replica_entry_info(slot, index, entry, starting_transaction_index);

        for plugin in plugin_manager.plugins.iter_mut() {
            if !plugin.entry_notifications_enabled() {
                continue;
            }
            // match plugin.notify_entry(entry) {
                match plugin.notify_entry(ReplicaEntryInfoVersions::V0_0_2(&entry_info)) {

                    Err(err) => {
                    error!(
                        "Failed to notify entry, error: ({}) to plugin {}",
                        err,
                        plugin.name()
                    )
                }
                Ok(_) => {
                    trace!("Successfully notified entry to plugin {}", plugin.name());
                }
            }
        }
        measure.stop();
        inc_new_counter_debug!(
            "geyser-plugin-notify_plugins_of_entry_info-us",
            measure.as_us() as usize,
            10000,
            10000
        );
    }

    // fn last_insert_entry(&self) -> u64 {
    //     let mut plugin_manager = self.plugin_manager.write().unwrap();
    //     if plugin_manager.plugins.is_empty() {
    //         return 0;
    //     }
    //
    //     for plugin in plugin_manager.plugins.iter_mut() {
    //         if !plugin.entry_notifications_enabled() {
    //             continue;
    //         }
    //         return plugin.last_insert_entry();
    //     }
    //
    //     0
    // }
}

impl EntryNotifierImpl {
    pub fn new(plugin_manager: Arc<RwLock<GeyserPluginManager>>) -> Self {
        Self { plugin_manager }
    }

    fn build_replica_entry_info(
        slot: Slot,
        index: usize,
        entry: &'_ EntrySummary,
        starting_transaction_index: usize,
    ) -> ReplicaEntryInfoV2<'_> {
        ReplicaEntryInfoV2 {
            slot,
            index,
            num_hashes: entry.num_hashes,
            hash: entry.hash.as_ref(),
            executed_transaction_count: entry.num_transactions,
            starting_transaction_index,
        }
    }
}
