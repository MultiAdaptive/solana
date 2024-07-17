use {
    solana_entry::entry::UntrustedEntry,
    std::sync::Arc,
};

/// Interface for notifying block untrusted entry changes
pub trait UntrustedEntryNotifier {
    /// Notify the untrusted entry
    fn notify_untrusted_entry(&self, entry: &UntrustedEntry);

    /// Query last entry slot
    fn last_insert_untrusted_entry(&self) -> u64;
}

pub type UntrustedEntryNotifierArc = Arc<dyn UntrustedEntryNotifier + Sync + Send>;

