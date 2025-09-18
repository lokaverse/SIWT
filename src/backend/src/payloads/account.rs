use crate::types::{CandidType, Deserialize};

#[cfg(feature = "ckbtc")]
use crate::types::{Principal, Set};

#[derive(CandidType, Deserialize)]
pub struct AccountDerivedAddressPayload {
    pub user: String,
    #[cfg(feature = "ckbtc")]
    pub ckbtc: AccountCkBtcPayload,
}

#[cfg(feature = "ckbtc")]
#[derive(CandidType, Deserialize)]
pub struct AccountCkBtcPayload {
    pub owners: Set<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct AccountDerivedBtcAddressPayload {
    pub user: String,
    #[cfg(feature = "ckbtc")]
    pub owners: Set<Principal>,
}
