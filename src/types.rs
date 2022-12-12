use std::fmt;

use serde::{Deserialize, Serialize};

const TXID_LENGTH: usize = 32;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RBFEvent {
    pub timestamp: u64,
    #[serde(
        deserialize_with = "hex::serde::deserialize",
        serialize_with = "hex::serde::serialize"
    )]
    pub replaced_txid: [u8; TXID_LENGTH],
    pub replaced_fee: u64,
    pub replaced_vsize: u64,
    pub replaced_entry_time: u64,
    #[serde(
        deserialize_with = "hex::serde::deserialize",
        serialize_with = "hex::serde::serialize"
    )]
    pub replaced_raw: Vec<u8>,
    #[serde(
        deserialize_with = "hex::serde::deserialize",
        serialize_with = "hex::serde::serialize"
    )]
    pub replacement_txid: [u8; TXID_LENGTH],
    pub replacement_fee: u64,
    pub replacement_vsize: u64,
    #[serde(
        deserialize_with = "hex::serde::deserialize",
        serialize_with = "hex::serde::serialize"
    )]
    pub replacement_raw: Vec<u8>,
}

impl fmt::Display for RBFEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction({}, fee={}, vsize={}) replaced with Transaction({}, fee={}, vsize={})",
            hex::encode(
                &self
                    .replaced_txid
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<u8>>()
            ),
            self.replaced_fee,
            self.replaced_vsize,
            hex::encode(
                &self
                    .replacement_txid
                    .iter()
                    .rev()
                    .cloned()
                    .collect::<Vec<u8>>()
            ),
            self.replacement_fee,
            self.replacement_vsize,
        )
    }
}
