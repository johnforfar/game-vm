use std::fmt;
use bytemuck::{ Pod, Zeroable };
use anchor_lang::prelude::*;

pub const SIGNATURE_BYTES: usize = 64;

// Note: As of April 25, 2024, the `anchor-lang (v30)` does not play well with
// solana_sdk::Signature. We need to define our own Signature struct
// unfortunately.

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct Signature {
    value: [u8; 64] // Using an explicit "value" here to avoid IDL generation issues
}

impl Default for Signature {
    fn default() -> Self {
        Self { value: [0u8; SIGNATURE_BYTES] }
    }
}

impl From<Signature> for [u8; SIGNATURE_BYTES] {
    fn from(from: Signature) -> Self {
        from.value
    }
}

impl From<[u8; SIGNATURE_BYTES]> for Signature {
    fn from(from: [u8; 64]) -> Self {
        Self { value: from }
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.value
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.value).into_string())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.value).into_string())
    }
}

impl Signature {
    pub const LEN: usize = SIGNATURE_BYTES;

    pub fn new(signature_slice: &[u8]) -> Self {
        Signature { value: <[u8; SIGNATURE_BYTES]>::try_from(signature_slice).unwrap() }
    }

    pub const fn new_from_array(signature_array: [u8; SIGNATURE_BYTES]) -> Self {
        Self { value: signature_array }
    }

    pub fn to_bytes(self) -> [u8; SIGNATURE_BYTES] {
        self.value
    }
}
