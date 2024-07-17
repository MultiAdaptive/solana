// use {
//     solana_entry::entry::UntrustedEntry,
//     std::sync::Arc,
// };
//
// /// Interface for notifying block entry changes
// pub trait EntryNotifier {
//     /// Notify the entry
//     fn notify_entry(&self, entry: &UntrustedEntry);
//
//     /// Query last entry slot
//     fn last_insert_entry(&self) -> u64;
// }
//
// pub type EntryNotifierArc = Arc<dyn EntryNotifier + Sync + Send>;
//
