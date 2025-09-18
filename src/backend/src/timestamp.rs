use crate::types::{fmt, CandidType, DateTime, Deserialize, Serialize, Utc};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    inner: DateTime<Utc>,
}

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        Timestamp {
            inner: DateTime::<Utc>::from_timestamp_nanos(value as i64),
        }
    }
}

impl From<Timestamp> for u64 {
    fn from(value: Timestamp) -> Self {
        value.inner.timestamp_nanos_opt().unwrap() as u64
    }
}

impl CandidType for Timestamp {
    fn _ty() -> candid::types::Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(&self.inner.to_string())
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'__de> Deserialize<'__de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'__de>,
    {
        DateTime::<Utc>::deserialize(deserializer).map(|dt| Timestamp { inner: dt })
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
