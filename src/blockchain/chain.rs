use std::sync::{Arc, RwLock};

use xblockchain_storage::StorageConfig;
use xblockchain_storage::{tag, Storage};

use super::chain_types::ChainTips;
use super::super::blockcfg::{GenesisData, BlockHash};

#[allow(dead_code)]
pub struct Blockchain {
    genesis_hash: BlockHash,
    /// the storage for the overall blockchains (blocks)
    storage: Storage,
    /// possible other known forks
    heads: ChainTips<BlockHash>,
    /// what we think is the real blockchain at this specific moment
    tip: BlockHash,
}

pub type BlockchainR = Arc<RwLock<Blockchain>>;

// FIXME: copied from xblockchain-cli
pub const LOCAL_BLOCKCHAIN_TIP_TAG : &'static str = "tip";

impl Blockchain {
    pub fn from_storage(genesis_data: &GenesisData, storage_config: &StorageConfig) -> Self {
        let storage = Storage::init(storage_config).unwrap();
        let genesis_hash = genesis_data.genesis_prev.clone();
        let tip = tag::read_hash(&storage, &LOCAL_BLOCKCHAIN_TIP_TAG).unwrap_or(genesis_hash.clone());
        Blockchain {
            genesis_hash,
            storage: storage,
            heads: ChainTips::new(),
            tip,
        }
    }

    /// return the latest
    pub fn get_tip(&self) -> BlockHash {
        self.tip.clone()
    }
}
