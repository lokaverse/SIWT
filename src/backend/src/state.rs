use crate::types::{
    fork_hash, labeled_hash, set_certified_data, states, AsHashTree, Hash, RbTree, RefCell,
    Signatures,
};

pub const LABEL_ASSETS: &[u8] = b"http_assets";
pub const LABEL_SIG: &[u8] = b"sig";

pub(crate) type AssetHashes = RbTree<&'static str, Hash>;

pub struct State {
    signatures: RefCell<Signatures>,
    assets: RefCell<AssetHashes>,
}

impl State {
    pub fn with<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        states::state(f)
    }

    pub fn signatures(&self) -> &RefCell<Signatures> {
        &self.signatures
    }

    pub fn assets(&self) -> &RefCell<AssetHashes> {
        &self.assets
    }

    pub fn update_root_hash(&self, signatures: &Signatures) {
        let assets = &*self.assets.borrow();

        let assets_root_hash = assets.root_hash();
        let signatures_root_hash = signatures.root_hash();
        let labeled_assets_hash = labeled_hash(LABEL_ASSETS, &assets_root_hash);
        let labeled_signatures_hash = labeled_hash(LABEL_SIG, &signatures_root_hash);
        let prefixed_root_hash = fork_hash(&labeled_assets_hash, &labeled_signatures_hash);

        set_certified_data(&prefixed_root_hash[..]);
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            signatures: RefCell::new(Signatures::default()),
            assets: RefCell::new(AssetHashes::default()),
        }
    }
}
