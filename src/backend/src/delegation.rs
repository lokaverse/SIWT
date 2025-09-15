use crate::responses::{Delegated, SignedDelegation};
use crate::types::{
    fork, hash, labeled, labeled_hash, utils, Accounts, AsHashTree, ByteBuf, CandidType, Hash,
    HashMap, HashTree, Principal, Serialize, Set, State, Timestamp,
    LABEL_ASSETS, LABEL_SIG,
};

#[derive(CandidType, Clone, Serialize)]
pub struct Delegation {
    user: String,
    pubkey: ByteBuf,
    session: ByteBuf,
    expiration: u64,
    targets: Set<Principal>,
}

impl Delegation {
    pub(crate) fn new(
        user: &str,
        session: &[u8],
        expiration: u64,
        targets: impl Iterator<Item = Principal>,
    ) -> Result<Self, String> {
        Ok(Delegation {
            user: user.to_string(),
            pubkey: Accounts::user_canister_public_key(user)?,
            session: ByteBuf::from(session),
            expiration,
            targets: targets.collect(),
        })
    }

    pub fn user(&self) -> &str {
        self.user.trim()
    }

    pub fn seed(&self) -> Hash {
        utils::seed(&self.user)
    }

    pub fn seed_hash(&self) -> Hash {
        hash::bytes(self.seed())
    }

    pub fn pubkey(&self) -> &[u8] {
        &self.pubkey
    }

    pub fn principal(&self) -> Principal {
        Principal::self_authenticating(self.pubkey())
    }

    pub fn session(&self) -> &[u8] {
        &self.session
    }

    pub fn session_hash(&self) -> Hash {
        hash::bytes(self.session())
    }

    fn set_session(mut self, session: &[u8]) -> Self {
        self.session = ByteBuf::from(session);
        self
    }

    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    #[allow(unused)]
    pub fn timestamp(&self) -> Timestamp {
        Timestamp::from(self.expiration)
    }

    pub fn targets(&self) -> impl Iterator<Item = &Principal> {
        self.targets.iter()
    }

    pub(crate) fn map(&self) -> HashMap<&str, hash::Value<'_>> {
        let mut map = HashMap::new();
        let mut targets = Vec::new();

        for principal in self.targets() {
            targets.push(hash::Value::Bytes(principal.as_ref()));
        }

        map.insert("pubkey", hash::Value::Bytes(&self.session));
        map.insert("expiration", hash::Value::U64(self.expiration));
        map.insert("targets", hash::Value::Array(targets));

        map
    }

    pub(crate) fn hash(&self) -> Hash {
        let hash = hash::map(self.map());

        hash::domain(b"ic-request-auth-delegation", &hash)
    }

    pub(crate) fn store(&self) -> Hash {
        State::with(|state| {
            let signatures = &mut *state.signatures().borrow_mut();

            let hash = signatures.put(self.seed_hash(), self.hash());

            state.update_root_hash(&signatures);

            Accounts::borrow_mut(|a| a.store(self.user(), &self.principal()));

            hash
        })
    }

    pub(crate) fn sign(&self, certificate: impl Into<ByteBuf>) -> Result<SignedDelegation, String> {
        let certificate = certificate.into();
        let hash = self.hash();

        State::with(|state| {
            let signatures = &mut *state.signatures().borrow_mut();

            let witness = signatures
                .witness(hash::bytes(self.seed()), hash)
                .ok_or(format!("Signature not found"))?;

            let reconstructed = witness.reconstruct();
            let root_hash = signatures.root_hash();

            if reconstructed != root_hash {
                return Err(format!(
                    "Witness hash mismatch: expected {}, got {}",
                    hex::encode(reconstructed),
                    hex::encode(root_hash)
                ));
            }

            let assets = &*state.assets().borrow();

            let tree = fork(
                HashTree::Pruned(labeled_hash(LABEL_ASSETS, &assets.root_hash())),
                labeled(LABEL_SIG, witness),
            );

            let certificate = CertificateSignature { certificate, tree };
            let signature = serialize_certificate(&certificate).map(ByteBuf::from)?;

            Ok(SignedDelegation {
                delegation: Delegated {
                    pubkey: self.session().to_vec().into(),
                    expiration: self.expiration(),
                    targets: self.targets.clone(),
                },
                signature,
                pubkey: self.pubkey.clone(),
            })
        })
    }
}

#[derive(Serialize)]
struct CertificateSignature<'a> {
    certificate: ByteBuf,
    tree: HashTree<'a>,
}

fn serialize_certificate<T: Serialize>(data: &T) -> Result<Vec<u8>, String> {
    let mut cbor_serializer = serde_cbor::ser::Serializer::new(Vec::new());

    cbor_serializer.self_describe().map_err(|e| e.to_string())?;

    data.serialize(&mut cbor_serializer)
        .map_err(|e| e.to_string())?;

    Ok(cbor_serializer.into_inner())
}
