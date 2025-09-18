use crate::types::*;

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct Message {
    user: String,
    session: ByteBuf,
    expiration: u64,
    canisters: Set<Principal>,
}

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct Messages {
    map: Map<Hash, Message>,
}

impl Messages {
    pub fn new() -> Messages {
        Messages { map: Map::new() }
    }

    pub fn prune(&mut self) -> &mut Self {
        let now = utils::now();

        self.map.retain(|_, message| message.expiration > now);
        self
    }

    pub fn insert(&mut self, message: Message) -> Option<Message> {
        self.map.insert(message.hash(), message)
    }

    #[allow(unused)]
    pub fn get(&self, hash: &Hash) -> Option<Message> {
        self.map.get(hash).cloned()
    }

    pub fn remove(&mut self, hash: &Hash) -> Option<Message> {
        self.map.remove(hash)
    }

    pub fn put(message: &Message) -> Option<Message> {
        Self::borrow_mut(|m| m.insert(message.clone()))
    }

    pub fn delete(hash: &Hash) -> Option<Message> {
        Self::borrow_mut(|m| m.remove(hash))
    }

    #[allow(unused)]
    pub fn borrow<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        states::messages::borrow(f)
    }

    pub fn borrow_mut<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        states::messages::borrow_mut(|messages| f(messages.prune()))
    }
}

impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}

impl Message {
    pub fn new(
        user: &str,
        session: impl AsRef<[u8]>,
        canisters: impl Iterator<Item = Principal>,
    ) -> Self {
        let setting = Setting::get();

        Self {
            user: user.to_owned(),
            session: ByteBuf::from(session.as_ref()),
            expiration: utils::now_add_minute(10),
            canisters: setting.canisters().copied().chain(canisters).collect(),
        }
    }

    pub fn user(&self) -> &str {
        &self.user
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

    pub fn to_vec(&self) -> Vec<u8> {
        let user = self.user.as_bytes().to_vec();
        let session = self.session.to_vec();
        let canisters = self
            .canisters()
            .flat_map(|p| p.as_slice())
            .cloned()
            .collect::<Vec<_>>();

        [user, session, canisters].concat().to_vec()
    }

    pub fn hash(&self) -> Hash {
        hash::bytes(&self.to_vec())
    }
}

impl Storable for Message {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }
}

impl From<&Message> for String {
    fn from(message: &Message) -> Self {
        let user = message.user();
        let canisters = message
            .canisters()
            .map(Principal::to_text)
            .collect::<Vec<_>>()
            .join(", ");

        format!("User {user} want to create delegation for accessing canister(s): {canisters}")
    }
}

impl From<Message> for String {
    fn from(value: Message) -> Self {
        Self::from(&value)
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(String::from(self).as_str())
    }
}
