use crate::types::*;

pub(crate) struct Accounts {
    users: StableBTreeMap<Principal, Hash, VM>,
    principals: StableBTreeMap<Hash, Principal, VM>,
    seeds: StableBTreeMap<Hash, String, VM>,
}

impl Accounts {
    fn new(memories: [VM; 3]) -> Self {
        let [m1, m2, m3] = memories;

        Self {
            users: StableBTreeMap::init(m1),
            principals: StableBTreeMap::init(m2),
            seeds: StableBTreeMap::init(m3),
        }
    }

    pub(crate) fn init(memories: [VM; 3]) -> RefCell<Self> {
        RefCell::new(Self::new(memories))
    }

    pub(crate) fn store(&mut self, user: &str, principal: &Principal) {
        let seed = hash::string(user);
        let user = user.to_owned();
        let principal = *principal;

        self.users.insert(principal, seed);
        self.principals.insert(seed, principal);
        self.seeds.insert(seed, user);
    }

    pub(crate) fn user(&self, principal: &Principal) -> Option<String> {
        self.users
            .get(principal)
            .map(|hash| self.seeds.get(&hash).unwrap())
    }

    pub(crate) fn principal(&self, user: &str) -> Option<Principal> {
        self.principals.get(&hash::string(user))
    }

    pub(crate) fn all(&self) -> impl Iterator<Item = (String, Principal)> {
        let users = self.users.iter().collect::<Map<_, _>>();
        let seeds = self.seeds.iter().collect::<Map<_, _>>();
        let mut map = Map::new();

        for (principal, hash) in users {
            let user = seeds.get(&hash).unwrap();

            map.insert(user.clone(), principal);
        }

        map.into_iter()
    }

    #[allow(unused)]
    pub(crate) fn borrow<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        states::accounts::borrow(f)
    }

    pub(crate) fn borrow_mut<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        states::accounts::borrow_mut(f)
    }

    pub(crate) fn user_canister_public_key(user: &str) -> Result<ByteBuf, String> {
        let canister = canister_principal().as_slice().to_vec();
        let seed = utils::seed(user);
        let mut bytes = Vec::new();

        bytes.push(canister.len() as u8);
        bytes.extend(canister);
        bytes.extend(seed);

        let algo = oid!(1, 3, 6, 1, 4, 1, 56387, 1, 2);
        let algo = ASN1Block::Sequence(0, vec![ASN1Block::ObjectIdentifier(0, algo)]);
        let subject = ASN1Block::BitString(0, bytes.len() * 8, bytes.to_vec());
        let info = ASN1Block::Sequence(0, vec![algo, subject]);
        let pubkey = to_der(&info).map_err(|e| e.to_string())?;

        Ok(ByteBuf::from(pubkey))
    }
}
