pub(crate) use std::borrow::Cow;
pub(crate) use std::cell::RefCell;
#[allow(unused)]
pub(crate) use std::collections::btree_map::{IntoIter as MapIntoIter, Iter as MapIter};
pub(crate) use std::collections::{BTreeMap as Map, BTreeSet as Set, BinaryHeap, HashMap};
pub(crate) use std::fmt;
#[allow(unused)]
pub(crate) use std::iter::{Cloned, Copied};

pub(crate) use candid::{CandidType, Principal};
pub(crate) use chrono::{DateTime, Utc};
pub(crate) use ic_cdk::api::{data_certificate, set_certified_data};
pub(crate) use ic_cdk::{caller as caller_principal, id as canister_principal};
pub(crate) use ic_certified_map::{fork, fork_hash, labeled, labeled_hash, leaf_hash};
pub(crate) use ic_certified_map::{AsHashTree, Hash, HashTree, RbTree};
pub(crate) use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
pub(crate) use ic_stable_structures::storable::{Bound, Storable};
pub(crate) use ic_stable_structures::{DefaultMemoryImpl, Memory, StableBTreeMap, StableCell};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use serde_bytes::ByteBuf;
pub(crate) use sha2::{Digest, Sha256};
pub(crate) use simple_asn1::{from_der, oid, to_der, ASN1Block};

pub use crate::accounts::Accounts;
pub use crate::delegation::Delegation;
pub use crate::globals::Globals;
pub use crate::messages::{Message, Messages};
pub use crate::setting::Setting;
pub use crate::signatures::Signatures;
pub use crate::state::{State, LABEL_ASSETS, LABEL_SIG};
pub use crate::timestamp::Timestamp;
pub use crate::{hash, states, utils};

pub(crate) type VM = VirtualMemory<DefaultMemoryImpl>;
pub(crate) type MemoryManagerType = RefCell<MemoryManager<DefaultMemoryImpl>>;
pub(crate) type SettingStateType = RefCell<StableCell<Setting, VM>>;
pub(crate) type AccountsStateType = RefCell<Accounts>;
