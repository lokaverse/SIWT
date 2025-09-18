use crate::types::{
    caller_principal, canister_principal, states, utils, Bound, CandidType, Cow, Deserialize,
    Memory, Principal, RefCell, Serialize, Set, StableCell, Storable,
};

#[derive(CandidType, Clone, Deserialize, Serialize)]
pub struct Setting {
    expiration_minute: u64,
    authorities: Set<Principal>,
    canisters: Set<Principal>,
}

impl Setting {
    pub(crate) fn init<M: Memory>(memory: M) -> RefCell<StableCell<Self, M>> {
        RefCell::new(StableCell::init(memory, Self::default()).unwrap())
    }

    pub fn expiration(&self) -> u64 {
        utils::now_add_minute(self.expiration_minute)
    }

    #[allow(unused)]
    pub fn expiration_minute(&self) -> u64 {
        self.expiration_minute
    }

    pub(crate) fn set_expiration_minute(&mut self, expiration_minute: u64) {
        self.expiration_minute = expiration_minute;
    }

    pub(crate) fn authorized(&self, principal: &Principal) -> bool {
        self.authorities.contains(principal)
    }

    #[allow(unused)]
    pub(crate) fn authorities(&self) -> impl Iterator<Item = &Principal> {
        self.authorities.iter()
    }

    pub(crate) fn canisters(&self) -> impl Iterator<Item = &Principal> {
        self.canisters.iter()
    }

    pub(crate) fn extends(
        &mut self,
        authorities: impl IntoIterator<Item = Principal>,
        canisters: impl IntoIterator<Item = Principal>,
    ) {
        self.authorities.extend(authorities);
        self.authorities.insert(caller_principal());
        self.canisters.extend(canisters);
        self.canisters.insert(canister_principal());
    }

    pub(crate) fn merge(self) {
        let mut setting = states::setting::get();

        setting.set_expiration_minute(self.expiration_minute);
        setting.extends(self.authorities, self.canisters);

        states::setting::set(setting);
    }

    pub(crate) fn get() -> Self {
        states::setting::get()
    }

    pub(crate) fn store(self) {
        states::setting::set(self);
    }
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            expiration_minute: 120,
            authorities: [caller_principal()].into(),
            canisters: [canister_principal()].into(),
        }
    }
}

impl Storable for Setting {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        serde_cbor::to_vec(self).unwrap().into()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(&bytes).unwrap()
    }
}
