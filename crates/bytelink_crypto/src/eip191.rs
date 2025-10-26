use crate::constant::EIP191_PREFIX;
use alloy_primitives::{B256, keccak256};

/// Create an [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatted message from the given byte slice.
/// ```text
/// ┌────────────────────────────────┬───────────┬───────────┐
/// │ \x19Bytelink Signed Message:\n │ len(msg)  │  payload  │
/// └────────────────────────────────┴───────────┴───────────┘
/// ```
pub fn eip191_message<T: AsRef<[u8]>>(message: T) -> Vec<u8> {
    fn eip191_message(message: &[u8]) -> Vec<u8> {
        let len = message.len();
        let mut len_string_buffer = itoa::Buffer::new();
        let len_string = len_string_buffer.format(len);

        let mut eth_message = Vec::with_capacity(EIP191_PREFIX.len() + len_string.len() + len);
        eth_message.extend_from_slice(EIP191_PREFIX.as_bytes());
        eth_message.extend_from_slice(len_string.as_bytes());
        eth_message.extend_from_slice(message);
        eth_message
    }

    eip191_message(message.as_ref())
}

/// Create a Keccak-256 hash of an [EIP-191](https://eips.ethereum.org/EIPS/eip-191) formatted message.
pub fn eip191_hash<T: AsRef<[u8]>>(message: T) -> B256 {
    keccak256(eip191_message(message))
}
