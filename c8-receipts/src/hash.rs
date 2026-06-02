use sha2::{Digest, Sha256};
use std::fmt;

/// SHA256-based cryptographic hash of a receipt.
///
/// A receipt hash is the identity proof of a single state transition.
/// It is computed deterministically from (pre_state || delta || post_state).
///
/// Hashes are used to build hash chains: each receipt's hash is committed to
/// by the next receipt's causal time or chain pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReceiptHash([u8; 32]);

impl ReceiptHash {
    /// Computes a SHA256 hash from raw bytes.
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result[..]);
        ReceiptHash(bytes)
    }

    /// Returns the hash as a 32-byte array.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns the hash as a hexadecimal string.
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for ReceiptHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

impl serde::Serialize for ReceiptHash {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.as_hex())
    }
}

impl<'de> serde::Deserialize<'de> for ReceiptHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Invalid hash length"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(ReceiptHash(array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_from_bytes_is_deterministic() {
        let data = b"test data";
        let h1 = ReceiptHash::from_bytes(data);
        let h2 = ReceiptHash::from_bytes(data);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_display() {
        let h = ReceiptHash::from_bytes(b"test");
        let hex_str = h.as_hex();
        assert!(hex_str.len() == 64, "SHA256 hex should be 64 chars");
    }
}
