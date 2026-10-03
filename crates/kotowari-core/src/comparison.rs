use crate::{change_records::FileChange, config::Config};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Blob {
    pub mode: String,
    pub bytes: Vec<u8>,
}

impl Blob {
    pub fn identity(&self) -> String {
        let mut hash = Sha256::new();
        hash.update(self.mode.as_bytes());
        hash.update([0]);
        hash.update(&self.bytes);
        format!("sha256:{:x}", hash.finalize())
    }
}

pub fn ir_identity(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

pub struct Comparison {
    pub base: String,
    pub target: String,
    pub config: Config,
    pub files: Vec<FileChange>,
    pub blobs: BTreeMap<String, Blob>,
}
