use crate::types::*;

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Middleware {
    pubkey: ByteBuf,
    session: ByteBuf,
    signature: ByteBuf,
    expiration: u64,
    canisters: Set<Principal>,
}

impl Middleware {
    pub fn new(
        pubkey: &[u8],
        session: &[u8],
        expiration: u64,
        canisters: impl Iterator<Item = Principal>,
    ) -> Self {
        Self {
            pubkey: ByteBuf::from(pubkey),
            session: ByteBuf::from(session),
            signature: ByteBuf::default(),
            expiration,
            canisters: canisters.collect(),
        }
    }

    pub fn sign(&mut self, signature: &[u8]) {
        self.signature = ByteBuf::from(signature);
    }
}

pub(crate) struct Middlewares {
    map: Map<Hash, Middleware>,
}

impl Middlewares {
    pub(crate) fn init() -> RefCell<Self> {
        RefCell::new(Self::default())
    }

    pub(crate) fn get(&self, hash: &Hash) -> Option<Middleware> {
        self.map.get(hash).cloned()
    }

    pub(crate) fn get_mut(&mut self, hash: &Hash) -> Option<&mut Middleware> {
        self.map.get_mut(hash)
    }

    pub(crate) fn store(&mut self, hash: Hash, middleware: Middleware) {
        self.map.insert(hash, middleware);
    }

    pub(crate) fn prune(&mut self) -> &mut Self {
        let now = utils::now();

        self.map.retain(|_, v| v.expiration > now);
        self
    }

    pub(crate) fn put(hash: Hash, middleware: impl Into<Middleware>) {
        Middlewares::borrow_mut(|m| m.store(hash, middleware.into()))
    }

    pub(crate) fn modify<F: FnOnce(&mut Middleware) -> R, R>(hash: &Hash, f: F) -> Option<R> {
        Middlewares::borrow_mut(|m| m.get_mut(hash).map(f))
    }

    pub(crate) fn borrow<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        states::middlewares::borrow(f)
    }

    pub(crate) fn borrow_mut<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        states::middlewares::borrow_mut(|m| f(m.prune()))
    }
}

impl Default for Middlewares {
    fn default() -> Self {
        Self { map: Map::new() }
    }
}

impl<'a> IntoIterator for &'a Middlewares {
    type Item = (&'a Hash, &'a Middleware);
    type IntoIter = MapIter<'a, Hash, Middleware>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl IntoIterator for Middlewares {
    type Item = (Hash, Middleware);
    type IntoIter = MapIntoIter<Hash, Middleware>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}
