use crate::types::{ByteBuf, CandidType, Hash, Message, Principal, Serialize, Set, Timestamp};

#[derive(CandidType, Clone, Serialize)]
pub struct Prepared {
    pub message: String,
    pub expiration: u64,
    pub expired: Timestamp,
    pub hash: Hash,
}

impl From<Message> for Prepared {
    fn from(message: Message) -> Self {
        Prepared {
            message: message.to_string(),
            expiration: message.expiration(),
            expired: message.expiration().into(),
            hash: message.hash(),
        }
    }
}

impl From<&Message> for Prepared {
    fn from(value: &Message) -> Self {
        Self::from(value.clone())
    }
}

#[derive(CandidType, Clone, Serialize)]
pub struct Login {
    pub expiration: u64,
    pub expired: Timestamp,
    pub canisters: Set<Principal>,
    pub hash: Hash,
}

#[derive(CandidType, Clone, Serialize)]
pub enum PrepareResponse {
    Ok(Prepared),
    Err(String),
}

impl Into<PrepareResponse> for Result<Prepared, String> {
    fn into(self) -> PrepareResponse {
        match self {
            Ok(prepared) => PrepareResponse::Ok(prepared),
            Err(err) => PrepareResponse::Err(err),
        }
    }
}

#[derive(CandidType, Clone, Serialize)]
pub enum LoginResponse {
    Ok(Login),
    Err(String),
}

impl Into<Result<Login, String>> for Login {
    fn into(self) -> Result<Login, String> {
        Ok(self)
    }
}

impl Into<LoginResponse> for Result<Login, String> {
    fn into(self) -> LoginResponse {
        match self {
            Ok(login) => LoginResponse::Ok(login),
            Err(err) => LoginResponse::Err(err),
        }
    }
}

#[derive(CandidType, Serialize)]
pub struct Delegated {
    pub pubkey: ByteBuf,
    pub expiration: u64,
    pub targets: Set<Principal>,
}

#[derive(CandidType, Serialize)]
pub struct SignedDelegation {
    pub delegation: Delegated,
    pub signature: ByteBuf,
    pub pubkey: ByteBuf,
}

#[derive(CandidType, Serialize)]
pub enum SignedDelegationResponse {
    Ok(SignedDelegation),
    Err(String),
}

impl Into<SignedDelegationResponse> for Result<SignedDelegation, String> {
    fn into(self) -> SignedDelegationResponse {
        match self {
            Ok(signed_delegation) => SignedDelegationResponse::Ok(signed_delegation),
            Err(err) => SignedDelegationResponse::Err(err),
        }
    }
}

impl Into<SignedDelegationResponse> for SignedDelegation {
    fn into(self) -> SignedDelegationResponse {
        SignedDelegationResponse::Ok(self)
    }
}
