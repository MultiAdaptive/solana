#[derive(Debug)]
pub(crate) struct UntrustedEntrySelector {
    pub select_all_entry: bool,
}

impl UntrustedEntrySelector {
    pub fn default() -> Self {
        Self {
            select_all_entry: false,
        }
    }

    pub fn new(enable: bool) -> Self {
        Self {
            select_all_entry: enable,
        }
    }

    /// Check if interest in shred
    pub fn is_enabled(&self) -> bool {
        self.select_all_entry
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn test_default_selector() {
        let untrusted_entry_selector = UntrustedEntrySelector::default();
        assert_eq!(false, untrusted_entry_selector.is_enabled())
    }

    #[test]
    fn test_enable_selector() {
        let untrusted_entry_selector = UntrustedEntrySelector::new(true);
        assert_eq!(true, untrusted_entry_selector.is_enabled())
    }
}
