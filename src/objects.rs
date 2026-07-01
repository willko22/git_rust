use sha2::{Digest, Sha256};


struct Blob {
    data: Vec<u8>,
}

impl Blob {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    fn hash(&self) -> String {
        let header = format!("blob {}\0", self.data.len());
        let mut payload = header.into_bytes();
        payload.extend(&self.data);

        let digest = Sha256::digest(&payload);
        digest.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

struct Tree {
    entries: Vec<(String, String)>, // (name, hash)
}

impl Tree {
    fn new(entries: Vec<(String, String)>) -> Self {
        Self { entries }
    }
    fn add(&mut self, name: String, hash: String) {
        self.entries.push((name, hash));
    }
    fn order(&mut self) {
        self.entries.sort_by(|a, b| a.0.cmp(&b.0));
    }

    fn hexToBytes(hash: &str) -> Vec<u8> {
        if hash.len() % 2 != 0 {
            panic!("Invalid SHA1 hex length");
        }

        let mut bytes = Vec::with_capacity(hash.len() / 2);
        for i in (0..hash.len()).step_by(2) {
            let byte = u8::from_str_radix(&hash[i..i + 2], 16)
                .expect("Invalid SHA1 hex character");
            bytes.push(byte);
        }

        bytes
    }
    
    // 
    fn getData(&mut self) -> Vec<u8> {
        // Real git tree entry: "<mode> <name>\0" + 20 raw hash bytes
        self.order();
        let mut data = Vec::new();
        for (name, hash) in &self.entries {
            let mode = "100644";
            data.extend(format!("{} {}\0", mode, name).into_bytes());
            data.extend(Self::hexToBytes(hash));
        }

        data
    }

    fn hash(&mut self) -> String {
        let data = self.getData();
        let header = format!("tree {}\0", data.len());
        let mut payload = header.into_bytes();
        payload.extend(data);

        let digest = Sha256::digest(&payload);
        digest.iter().map(|b| format!("{:02x}", b)).collect()
    }
}