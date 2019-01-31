use super::config::{ConsensusConfig, ValidatorKeys};

/// The initial configuration which is committed into the genesis block.
///
/// The genesis block is the first block in the blockchain which is created
/// when the blockchain is initially launched. This block can contain some service
/// data, but does not include transactions.
///
/// `GenesisConfig` includes consensus related configuration and the public keys of validators.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GenesisConfig {
    /// Consensus configuration.
    pub consensus: ConsensusConfig,
    /// List of public keys of validators.
    pub validator_keys: Vec<ValidatorKeys>,
}

impl GenesisConfig {
    /// Creates a default configuration from the given list of public keys.
    pub fn new<I: Iterator<Item = ValidatorKeys>>(validators: I) -> Self {
        Self::new_with_consensus(ConsensusConfig::default(), validators)
    }

    /// Creates a configuration from the given consensus configuration and list of public keys.
    pub fn new_with_consensus<I>(consensus: ConsensusConfig, validator_keys: I) -> Self
    where
        I: Iterator<Item = ValidatorKeys>,
    {
        consensus.warn_if_nonoptimal();
        Self {
            consensus,
            validator_keys: validator_keys.collect(),
        }
    }
}
