use crate::types::*;

#[derive(Default)]
pub(crate) struct Unit;

impl AsHashTree for Unit {
    fn root_hash(&self) -> Hash {
        leaf_hash(&b""[..])
    }
    fn as_hash_tree(&self) -> HashTree<'_> {
        HashTree::Leaf(Cow::from(&b""[..]))
    }
}

#[derive(PartialEq, Eq)]
struct SigExpiration {
    seed: Hash,
    hash: Hash,
    expiration: u64,
}

impl Ord for SigExpiration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.expiration.cmp(&self.expiration)
    }
}

impl PartialOrd for SigExpiration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct Signatures {
    certified: RbTree<Hash, RbTree<Hash, Unit>>,
    expiration: BinaryHeap<SigExpiration>,
}

impl Signatures {
    pub fn put(&mut self, seed: Hash, hash: Hash) -> Hash {
        self.prune();

        let setting = Setting::get();
        let expiration = setting.expiration();

        if self.certified.get(&seed[..]).is_none() {
            let mut map = RbTree::new();

            map.insert(hash, Unit);

            self.certified.insert(seed, map);
        } else {
            self.certified.modify(&seed[..], |map| {
                map.insert(hash, Unit);
            });
        }
        self.expiration.push(SigExpiration {
            seed,
            hash,
            expiration,
        });

        hash
    }

    pub fn puts<const N: usize>(&mut self, seed: Hash, hashes: [Hash; N]) -> [Hash; N] {
        self.prune();

        let setting = Setting::get();
        let expiration = setting.expiration();

        if self.certified.get(&seed[..]).is_none() {
            self.certified
                .insert(seed, RbTree::from_iter(hashes.map(|hash| (hash, Unit))));
        } else {
            self.certified.modify(&seed[..], |map| {
                for hash in hashes {
                    map.insert(hash, Unit);
                }
            });
        }

        self.expiration.extend(hashes.map(|hash| SigExpiration {
            seed,
            hash,
            expiration,
        }));

        hashes
    }

    pub fn delete(&mut self, seed: Hash, delegation: Hash) {
        let mut is_empty = false;

        self.certified.modify(&seed[..], |m| {
            m.delete(&delegation[..]);
            is_empty = m.is_empty();
        });

        if is_empty {
            self.certified.delete(&seed[..]);
        }
    }

    pub fn prune(&mut self) -> usize {
        let now = utils::now();
        let mut pruned = 0;
        let pruning = std::cmp::min(10, self.expiration.len());

        for _ in 0..pruning {
            if let Some(expiration) = self.expiration.peek() {
                if expiration.expiration > now {
                    return pruned;
                }
            }

            if let Some(expiration) = self.expiration.pop() {
                self.delete(expiration.seed, expiration.hash);
            }

            pruned += 1;
        }

        pruned
    }

    pub fn root_hash(&self) -> Hash {
        self.certified.root_hash()
    }

    pub fn witness(&self, seed_hash: Hash, delegation_hash: Hash) -> Option<HashTree<'_>> {
        self.certified
            .get(&seed_hash[..])?
            .get(&delegation_hash[..])?;

        let witness = self.certified.nested_witness(&seed_hash[..], |nested| {
            nested.witness(&delegation_hash[..])
        });

        Some(witness)
    }
}
