use crypto::{CryptoHash, Hash};

/// A common trait for the ability to compute a unique hash.
///
/// Unlike `CryptoHash`, the hash value returned by the `UniqueHash::hash()`
/// method isn't always irreversible. This hash is used, for example, in the
/// storage as a key, as uniqueness is important in this case.
pub trait UniqueHash {
    /// Returns a hash of the value.
    ///
    /// Hash must be unique, but not necessary cryptographic.
    fn hash(&self) -> Hash;
}

impl<T: CryptoHash> UniqueHash for T {
    fn hash(&self) -> Hash {
        CryptoHash::hash(self)
    }
}
