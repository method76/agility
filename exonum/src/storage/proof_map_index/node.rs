#![allow(unsafe_code)]

use std::borrow::Cow;

use super::{
    super::{StorageKey, StorageValue},
    key::{ChildKind, ProofPath, PROOF_PATH_SIZE},
};
use crypto::{hash, CryptoHash, Hash, HASH_SIZE};

const BRANCH_NODE_SIZE: usize = 2 * (HASH_SIZE + PROOF_PATH_SIZE);

#[derive(Debug)]
pub enum Node<T: StorageValue> {
    Leaf(T),
    Branch(BranchNode),
}

#[derive(Clone)]
pub struct BranchNode {
    raw: Vec<u8>,
}

impl BranchNode {
    pub fn empty() -> Self {
        Self {
            raw: vec![0; BRANCH_NODE_SIZE],
        }
    }

    pub fn child_hash(&self, kind: ChildKind) -> &Hash {
        unsafe {
            let from = match kind {
                ChildKind::Right => HASH_SIZE,
                ChildKind::Left => 0,
            };
            &*(&self.raw[from] as *const u8 as *const Hash)
        }
    }

    pub fn child_path(&self, kind: ChildKind) -> ProofPath {
        let from = match kind {
            ChildKind::Right => 2 * HASH_SIZE + PROOF_PATH_SIZE,
            ChildKind::Left => 2 * HASH_SIZE,
        };
        ProofPath::read(&self.raw[from..from + PROOF_PATH_SIZE])
    }

    pub fn set_child_path(&mut self, kind: ChildKind, prefix: &ProofPath) {
        let from = match kind {
            ChildKind::Right => 2 * HASH_SIZE + PROOF_PATH_SIZE,
            ChildKind::Left => 2 * HASH_SIZE,
        };
        prefix.write(&mut self.raw[from..from + PROOF_PATH_SIZE]);
    }

    pub fn set_child_hash(&mut self, kind: ChildKind, hash: &Hash) {
        let from = match kind {
            ChildKind::Right => HASH_SIZE,
            ChildKind::Left => 0,
        };
        self.raw[from..from + HASH_SIZE].copy_from_slice(hash.as_ref());
    }

    pub fn set_child(&mut self, kind: ChildKind, prefix: &ProofPath, hash: &Hash) {
        self.set_child_path(kind, prefix);
        self.set_child_hash(kind, hash);
    }
}

impl CryptoHash for BranchNode {
    fn hash(&self) -> Hash {
        hash(&self.raw)
    }
}

impl StorageValue for BranchNode {
    fn into_bytes(self) -> Vec<u8> {
        self.raw
    }

    fn from_bytes(value: Cow<[u8]>) -> Self {
        Self {
            raw: value.into_owned(),
        }
    }
}

impl ::std::fmt::Debug for BranchNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("BranchNode")
            .field("left_path", &self.child_path(ChildKind::Left))
            .field("left_hash", &self.child_hash(ChildKind::Left))
            .field("right_path", &self.child_path(ChildKind::Right))
            .field("right_hash", &self.child_hash(ChildKind::Right))
            .field("hash", &self.hash())
            .finish()
    }
}

#[test]
fn test_branch_node() {
    let mut branch = BranchNode::empty();

    let lh = hash(&[1, 2]);
    let rh = hash(&[3, 4]);
    let ls = ProofPath::new(&[253; 32]);
    let rs = ProofPath::new(&[244; 32]);

    branch.set_child(ChildKind::Left, &ls, &lh);
    branch.set_child(ChildKind::Right, &rs, &rh);

    assert_eq!(branch.child_hash(ChildKind::Left), &lh);
    assert_eq!(branch.child_hash(ChildKind::Right), &rh);
    assert_eq!(branch.child_path(ChildKind::Left), ls);
    assert_eq!(branch.child_path(ChildKind::Right), rs);
}
