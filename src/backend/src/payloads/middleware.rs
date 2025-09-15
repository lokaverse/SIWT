use crate::types::{from_der, ByteBuf, CandidType, Delegation, Deserialize, Hash, Principal};

use super::DelegationPayload;

#[derive(CandidType, Deserialize)]
pub(crate) struct GetMiddlewareSessionPayload {
    hash: Hash,
}

#[derive(CandidType, Deserialize)]
pub(crate) struct MiddlewareStoreSignaturePayload {
    pub hash: Hash,
    pub signature: ByteBuf,
}

#[derive(CandidType, Deserialize)]
pub(crate) struct MiddlewareLoginPayload {
    hash: Hash,
    middleware: ByteBuf,
}

impl MiddlewareLoginPayload {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn middleware(&self) -> &[u8] {
        &self.middleware
    }
}

#[derive(CandidType, Deserialize)]
pub(crate) struct MiddlewareDelegationPayload {
    request: DelegationPayload,
    middleware: ByteBuf,
}

impl MiddlewareDelegationPayload {
    pub fn user(&self) -> &str {
        self.request.user()
    }

    pub fn session(&self) -> &[u8] {
        self.request.session()
    }

    pub fn expiration(&self) -> u64 {
        self.request.expiration()
    }

    pub fn canisters(&self) -> impl Iterator<Item = &Principal> {
        self.request.canisters()
    }

    pub fn middleware(&self) -> &[u8] {
        &self.middleware
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.user().is_empty() {
            return Err(format!("User is empty"));
        }

        if self.session().is_empty() {
            return Err(format!("Session public key is empty"));
        }

        if let Err(e) = from_der(self.session()) {
            return Err(format!("Session public key is invalid: {}", e));
        }

        if self.middleware().is_empty() {
            return Err(format!("Middleware public key is empty"));
        }

        if let Err(e) = from_der(self.middleware()) {
            return Err(format!("Middleware public key is invalid: {}", e));
        }

        Ok(())
    }

    pub fn into_session_delegation(&self) -> Result<Delegation, String> {
        self.request.into_delegation()
    }

    pub fn into_middleware_delegation(&self) -> Result<Delegation, String> {
        Delegation::new(
            self.user(),
            self.middleware(),
            self.expiration(),
            self.canisters().copied(),
        )
    }
}
