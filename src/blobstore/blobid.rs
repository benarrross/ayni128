#![allow(unused)]
use std::cmp::{Ordering, PartialEq, PartialOrd, Ord, Eq};
use std::fmt;


// NYI: use an explicit std::num::NonZeroU64 here instead?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobId(pub std::num::NonZeroU64);


impl BlobId {

    #[inline]
    pub fn new(value: std::num::NonZeroU64) -> BlobId { BlobId { 0: value } }

    #[inline]
    pub fn value(&self) -> std::num::NonZeroU64 { self.0 }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        let value: u64 = self.0.into();
        value.to_le_bytes()
    }
}


impl From<std::num::NonZeroU64> for BlobId {

    #[inline]
    fn from(value: std::num::NonZeroU64) -> Self {
        BlobId { 0: value as std::num::NonZeroU64 }
    }
}

impl fmt::Display for BlobId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({})", self.0)
    }
}