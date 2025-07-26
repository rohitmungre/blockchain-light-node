use sled::Db;
use sha2::{Digest, Sha256};
use anyhow::Result;

/// A simple key–value store mapping height → header bytes
pub struct HeaderStore {
    db: Db,
}

impl HeaderStore {
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn put(&self, height: u64, header: &[u8]) -> Result<()> {
        self.db.insert(height.to_be_bytes(), header)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, height: u64) -> Option<Vec<u8>> {
        self.db.get(height.to_be_bytes()).ok().flatten().map(|ivec| ivec.to_vec())
    }

    pub fn last_height(&self) -> Option<u64> {
        self.db.iter().keys().last().and_then(|res| res.ok()).map(|k| u64::from_be_bytes(k.as_ref().try_into().unwrap()))
    }

    pub fn verify_header(header: &[u8], expected_hash: &str) -> bool {
        let hash = Sha256::digest(header);
        hex::encode(hash) == expected_hash
    }
}
