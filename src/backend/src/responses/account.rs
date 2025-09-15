use crate::types::{ByteBuf, CandidType, Principal, Serialize};

#[cfg(feature = "ckbtc")]
use crate::types::Map;

#[derive(CandidType, Serialize)]
pub enum AccountDerivedAddressResponse {
    Ok(AccountDerivedAddress),
    Err(String),
}

#[derive(CandidType, Serialize)]
pub struct AccountDerivedAddress {
    pub pubkey: ByteBuf,
    pub principal: Principal,
    #[cfg(feature = "ckbtc")]
    pub btc: AccountDerivedBtcAddress,
}

#[cfg(feature = "ckbtc")]
#[derive(CandidType, Serialize)]
pub struct AccountDerivedBtcAddress {
    pub address: String,
    pub accounts: Map<Principal, Option<String>>,
}

impl Into<AccountDerivedAddressResponse> for Result<AccountDerivedAddress, String> {
    fn into(self) -> AccountDerivedAddressResponse {
        match self {
            Ok(address) => AccountDerivedAddressResponse::Ok(address),
            Err(e) => AccountDerivedAddressResponse::Err(e),
        }
    }
}
