use crate::types::{
    canister_principal, from_der, ByteBuf, CandidType, Copied, Delegation, Deserialize, Hash,
    Message, Principal, Set,
};

#[derive(CandidType, Deserialize)]
pub struct PreparePayload {
    user: String,
    session: ByteBuf,
    canisters: Set<Principal>,
}

impl PreparePayload {
    pub fn user(&self) -> &str {
        self.user.trim()
    }

    pub fn canisters(&self) -> Copied<impl Iterator<Item = &Principal>> {
        self.canisters.iter().copied()
    }

    pub fn session(&self) -> &[u8] {
        &self.session
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

        Ok(())
    }
}

impl Into<Message> for PreparePayload {
    fn into(self) -> Message {
        Message::new(
            self.user(),
            self.session(),
            self.canisters().chain([canister_principal()]),
        )
    }
}

#[derive(CandidType, Deserialize)]
pub struct LoginPayload {
    hash: Hash,
}

impl LoginPayload {
    pub fn hash(&self) -> &Hash {
        &self.hash
    }
}

#[derive(CandidType, Deserialize)]
pub struct DelegationPayload {
    user: String,
    session: ByteBuf,
    expiration: u64,
    canisters: Set<Principal>,
}

impl DelegationPayload {
    pub fn user(&self) -> &str {
        self.user.trim()
    }

    pub fn session(&self) -> &[u8] {
        &self.session
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    pub fn canisters(&self) -> impl Iterator<Item = &Principal> {
        self.canisters.iter()
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

        Ok(())
    }

    pub fn into_delegation(&self) -> Result<Delegation, String> {
        Delegation::new(
            self.user(),
            self.session(),
            self.expiration(),
            self.canisters().copied(),
        )
    }
}
