#[derive(Default, Clone, Debug)]
pub struct SMTTransaction {}

impl SMTTransaction {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes
    }
}


impl From<Vec<u8>> for SMTTransaction {
    fn from(bytes: Vec<u8>) -> Self {
        Self {}
    }
}

impl Into<Vec<u8>> for SMTTransaction {
    fn into(self) -> Vec<u8> {
        self.to_vec()
    }
}
