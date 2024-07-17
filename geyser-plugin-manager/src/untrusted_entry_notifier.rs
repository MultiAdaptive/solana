/// Module responsible for notifying plugins about entries
use {
    crate::{
        geyser_plugin_manager::GeyserPluginManager,
        untrusted_entry_notifier_interface::UntrustedEntryNotifier},
    log::*,
    solana_entry::entry::UntrustedEntry,
    solana_measure::measure::Measure,
    solana_metrics::*,
    std::sync::{Arc, RwLock},
};
use solana_entry::entry::Entry;

pub(crate) struct UntrustedEntryNotifierImpl {
    plugin_manager: Arc<RwLock<GeyserPluginManager>>,
}

impl UntrustedEntryNotifier for UntrustedEntryNotifierImpl {
    /// Notify the entry
    fn notify_untrusted_entry(&self, entry: &UntrustedEntry) {
        let mut measure = Measure::start("geyser-plugin-notify_plugins_of_untrusted_entry_info");
        let mut plugin_manager = self.plugin_manager.write().unwrap();
        if plugin_manager.plugins.is_empty() {
            return;
        }

        for plugin in plugin_manager.plugins.iter_mut() {
            if !plugin.untrusted_entry_notifications_enabled() {
                continue;
            }
            match plugin.notify_untrusted_entry(entry) {
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
            "geyser-plugin-notify_plugins_of_untrusted_entry_info-us",
            measure.as_us() as usize,
            10000,
            10000
        );
    }

    fn last_insert_untrusted_entry(&self) -> u64 {
        let mut plugin_manager = self.plugin_manager.write().unwrap();
        if plugin_manager.plugins.is_empty() {
            return 0;
        }

        for plugin in plugin_manager.plugins.iter_mut() {
            if !plugin.untrusted_entry_notifications_enabled() {
                continue;
            }
            return plugin.last_insert_untrusted_entry();
        }

        0
    }
}

impl UntrustedEntryNotifierImpl {
    pub fn new(plugin_manager: Arc<RwLock<GeyserPluginManager>>) -> Self {
        Self { plugin_manager }
    }

    fn build_untrusted_entry(
        entries: Vec<Entry>,
        slot: u64,
        parent_slot: u64,
        is_full_slot: bool,
    ) -> UntrustedEntry {
        UntrustedEntry {
            entries: entries,
            slot: slot,
            parent_slot: parent_slot,
            is_full_slot: is_full_slot,
        }
    }
}
