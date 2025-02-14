use serde::Deserialize;
use serde::Serialize;

impl_tryfrom_serde_value!(JanusId);

/// Mountpoints, Rooms and Participants Identifier.
///
/// Identifier should be by default unsigned integer, unless configured otherwise in the plugin config.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JanusId {
    /// String Identifier
    String(String),
    /// Unsigned Integer Identifier
    Uint(U63),
}

#[cfg(feature = "ffi-compatible")]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct U63 {
    pub inner: u64,
}

#[cfg(not(feature = "ffi-compatible"))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct U63(u64);

impl U63 {
    pub const MAX: u64 = (1 << 63) - 1;

    pub fn new(value: u64) -> Self {
        Self::new_wrapping(value)
    }

    #[cfg(feature = "ffi-compatible")]
    pub fn new_wrapping(value: u64) -> Self {
        Self {
            inner: value & U63::MAX,
        }
    }

    #[cfg(feature = "ffi-compatible")]
    pub fn new_saturating(value: u64) -> Self {
        if value > U63::MAX {
            Self { inner: U63::MAX }
        } else {
            Self { inner: value }
        }
    }

    #[cfg(not(feature = "ffi-compatible"))]
    pub fn new_wrapping(value: u64) -> Self {
        Self(value & U63::MAX)
    }

    #[cfg(not(feature = "ffi-compatible"))]
    pub fn new_saturating(value: u64) -> Self {
        if value > U63::MAX {
            Self(U63::MAX)
        } else {
            Self(value)
        }
    }
}

impl From<u64> for U63 {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "ffi-compatible")]
impl Serialize for U63 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(not(feature = "ffi-compatible"))]
impl Serialize for U63 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for U63 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(U63::new(value))
    }
}
