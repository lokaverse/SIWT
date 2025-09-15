use crate::types::*;

type Pair = (String, ByteBuf);

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Global {
    key: String,
    value: ByteBuf,
}

pub(crate) struct Globals(StableBTreeMap<Hash, Global, VM>);

impl Global {
    pub fn hash(&self) -> Hash {
        hash::string(&self.key)
    }

    pub fn pair(&self) -> Pair {
        (self.key.clone(), self.value.clone())
    }
}

impl Into<Pair> for Global {
    fn into(self) -> Pair {
        self.pair()
    }
}

impl Into<Pair> for &Global {
    fn into(self) -> Pair {
        self.pair()
    }
}

impl Into<ByteBuf> for Global {
    fn into(self) -> ByteBuf {
        self.value
    }
}

impl Into<ByteBuf> for &Global {
    fn into(self) -> ByteBuf {
        self.value.clone()
    }
}

impl<K, V> From<(K, V)> for Global
where
    K: AsRef<str>,
    V: AsRef<[u8]>,
{
    fn from((k, v): (K, V)) -> Self {
        Self {
            key: k.as_ref().to_string(),
            value: ByteBuf::from(v.as_ref()),
        }
    }
}

impl Storable for Global {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }
}

impl Globals {
    pub fn new(memory: VM) -> Self {
        Self(StableBTreeMap::init(memory))
    }

    pub fn init(memory: VM) -> RefCell<Self> {
        RefCell::new(Self::new(memory))
    }

    pub fn all() -> impl Iterator<Item = Pair> {
        let globals = Self::borrow(|Self(m)| m.iter().collect::<Map<_, _>>());

        globals.into_iter().map(|(_, g)| g.into())
    }

    pub fn get(key: impl AsRef<str>) -> Option<ByteBuf> {
        Self::borrow(|Self(map)| map.get(&hash::string(key.as_ref())).map(Into::into))
    }

    pub fn of(keys: impl Iterator<Item = impl AsRef<str>>) -> impl Iterator<Item = Pair> {
        Self::borrow(|Self(map)| {
            keys.filter_map(|k| map.get(&hash::string(k.as_ref())).map(Into::into))
                .collect::<Map<_, _>>()
        })
        .into_iter()
    }

    pub fn store(key: impl AsRef<str>, value: impl AsRef<[u8]>) -> Option<Pair> {
        let global = Global::from((key, value));

        Self::borrow_mut(|Self(map)| map.insert(global.hash(), global).map(Into::into))
    }

    pub fn stores(
        values: impl Iterator<Item = (impl AsRef<str>, impl AsRef<[u8]>)>,
    ) -> impl Iterator<Item = Pair> {
        Self::borrow_mut(|Self(map)| {
            values
                .map(|(key, value)| {
                    let global = Global::from((key, value));

                    map.insert(global.hash(), global.clone());

                    global.into()
                })
                .collect::<Map<_, _>>()
        })
        .into_iter()
    }

    pub fn keys() -> impl Iterator<Item = String> {
        Self::borrow(|Self(map)| map.iter().map(|(_, g)| g.key).collect::<Set<_>>()).into_iter()
    }

    pub fn has(key: impl AsRef<str>) -> bool {
        Self::borrow(|Self(map)| map.contains_key(&hash::string(key.as_ref())))
    }

    pub fn contains(mut keys: impl Iterator<Item = impl AsRef<str>>) -> bool {
        Self::borrow(|Self(m)| keys.all(|k| m.contains_key(&hash::string(k.as_ref()))))
    }

    pub fn remove(key: impl AsRef<str>) -> Option<ByteBuf> {
        Self::borrow_mut(|Self(map)| map.remove(&hash::string(key.as_ref())).map(Into::into))
    }

    pub fn removes(keys: impl Iterator<Item = impl AsRef<str>>) -> impl Iterator<Item = Pair> {
        Self::borrow_mut(|Self(map)| {
            keys.filter_map(|key| map.remove(&hash::string(key.as_ref())).map(Into::into))
                .collect::<Map<_, _>>()
        })
        .into_iter()
    }

    fn borrow<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        states::globals::borrow(f)
    }

    fn borrow_mut<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        states::globals::borrow_mut(f)
    }
}
