use crate::types::{hash, Hash};

pub(crate) fn now() -> u64 {
    ic_cdk::api::time()
}

pub(crate) fn now_add_minute(minute: u64) -> u64 {
    let minute = minute.saturating_mul(60).saturating_mul(1_000_000_000);

    now().saturating_add(minute)
}

pub(crate) fn seed(user: &str) -> Hash {
    let length = hash::u64(user.len() as u64);
    let mut seed = Vec::new();

    seed.extend(length);
    seed.extend(user.as_bytes());

    hash::bytes(seed)
}
