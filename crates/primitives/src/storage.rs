use super::{H256, U256};
use reth_codecs::{derive_arbitrary, Compact};
use serde::{Deserialize, Serialize};

/// Account storage entry.
#[derive_arbitrary(compact)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct StorageEntry {
    /// Storage key.
    pub key: H256,
    /// Value on storage key.
    pub value: U256,
}

// NOTE: Removing main_codec and manually encode subkey
// and compress second part of the value. If we have compression
// over whole value (Even SubKey) that would mess up fetching of values with seek_by_key_subkey
impl Compact for StorageEntry {
    fn to_compact(self, buf: &mut impl bytes::BufMut) -> usize {
        // for now put full bytes and later compress it.
        buf.put_slice(&self.key.to_fixed_bytes()[..]);
        self.value.to_compact(buf) + 32
    }

    fn from_compact(buf: &[u8], len: usize) -> (Self, &[u8])
    where
        Self: Sized,
    {
        let key = H256::from_slice(&buf[..32]);
        let (value, out) = U256::from_compact(&buf[32..], len - 32);
        (Self { key, value }, out)
    }
}
