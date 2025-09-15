use crate::types::{CandidType, Deserialize, Principal, Set};

#[derive(CandidType, Deserialize)]
pub struct SettingExtendsPayload {
    pub(crate) authorities: Set<Principal>,
    pub(crate) canisters: Set<Principal>,
}
