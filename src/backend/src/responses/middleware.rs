use crate::types::{ByteBuf, CandidType, Principal, Serialize, Set};

#[derive(CandidType, Serialize)]
pub enum MiddlewareSignedDelegationResponse {
    Ok(MiddlewareSignedDelegation),
    Err(String),
}

#[derive(CandidType, Serialize)]
pub struct MiddlewareSignedDelegation {
    pub session: MiddlewareDelegated,
    pub middleware: MiddlewareDelegated,
    pub expiration: u64,
    pub canisters: Set<Principal>,
    pub pubkey: ByteBuf,
}

#[derive(CandidType, Serialize)]
pub struct MiddlewareDelegated {
    pub pubkey: ByteBuf,
    pub signature: ByteBuf,
}

impl Into<MiddlewareSignedDelegationResponse> for Result<MiddlewareSignedDelegation, String> {
    fn into(self) -> MiddlewareSignedDelegationResponse {
        match self {
            Ok(signed) => MiddlewareSignedDelegationResponse::Ok(signed),
            Err(e) => MiddlewareSignedDelegationResponse::Err(e),
        }
    }
}
