use blockchain::Blockchain;
use crypto::{PublicKey, SecretKey};
use node::ApiSender;
use storage::Snapshot;

/// Provides the current blockchain state to API handlers.
///
/// This structure is a part of the node that is available to the API. For example,
/// it can return the private key of the node, which allows the service to send
/// certain transactions to the blockchain. This case is used in the Exonum
/// [Configuration Updater service](https://exonum.com/doc/advanced/configuration-updater/).
#[derive(Debug, Clone)]
pub struct ServiceApiState {
    blockchain: Blockchain,
}

impl ServiceApiState {
    /// Constructs state for the given blockchain.
    pub fn new(blockchain: Blockchain) -> Self {
        Self { blockchain }
    }

    /// Returns a reference to the blockchain of this node.
    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// Creates a read-only snapshot of the current blockchain state.
    pub fn snapshot(&self) -> Box<dyn Snapshot> {
        self.blockchain.snapshot()
    }

    /// Returns the public key of the current node.
    pub fn public_key(&self) -> &PublicKey {
        &self.blockchain.service_keypair.0
    }

    /// Returns the secret key of the current node.
    pub fn secret_key(&self) -> &SecretKey {
        &self.blockchain.service_keypair.1
    }

    /// Returns a reference to the API sender.
    pub fn sender(&self) -> &ApiSender {
        &self.blockchain.api_sender
    }
}
