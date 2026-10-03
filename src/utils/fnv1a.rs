use const_fnv1a_hash::{fnv1a_hash_64, fnv1a_hash_str_64};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Fnv1aHash(u64);

impl Fnv1aHash {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self::from_str(s.as_ref())
    }

    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub const fn from_str(s: &str) -> Self {
        Self(fnv1a_hash_str_64(s))
    }

    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self(fnv1a_hash_64(bytes, None))
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for Fnv1aHash {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<&str> for Fnv1aHash {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<&[u8]> for Fnv1aHash {
    #[inline]
    fn from(value: &[u8]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<Fnv1aHash> for u64 {
    #[inline]
    fn from(value: Fnv1aHash) -> Self {
        value.0
    }
}
