#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xblockchain;
extern crate xblockchain_storage;
extern crate exe_common;

extern crate xchain;

use std::path::{PathBuf};

use xchain::{gclock, state};
use xchain::state::State;
use xchain::tpool::{TPool};
use xchain::blockchain::{Blockchain};
use xchain::utils::{task_create, task_create_with_inputs, Task};

use std::sync::{Arc, RwLock, mpsc::Receiver};
use std::{time, thread};

use xblockchain::tx::{TxId, TxAux};
use xblockchain_storage::StorageConfig;

pub type TODO = u32;
pub type BlockchainR = Arc<RwLock<Blockchain>>;
pub type TPoolR = Arc<RwLock<TPool<TxId, TxAux>>>;

fn transaction_task(_tpool: TPoolR, r: Receiver<TODO>) {
    loop {
        let tquery = r.recv().unwrap();
        println!("transaction received: {}", tquery)
    }
}

fn block_task(_blockchain: BlockchainR, r: Receiver<TODO>) {
    loop {
        let tquery = r.recv().unwrap();
        println!("transaction received: {}", tquery)
    }
}

fn client_task(_blockchain: BlockchainR, r: Receiver<TODO>) {
    loop {
        let query = r.recv().unwrap();
        println!("client query received: {}", query)
    }
}

fn main() {
    // # load parameters & config
    //
    // parse the command line arguments, the config files supplied
    // and setup the initial values
    let mut state = State::new();

    let pathbuf = PathBuf::from(r"pool-storage"); // FIXME HARDCODED should come from config
    let storage_config = StorageConfig::new(&pathbuf);
    let blockchain_data = Blockchain::from_storage(&storage_config);
    let blockchain = Arc::new(RwLock::new(blockchain_data));

    // # Bootstrap phase
    //
    // done at every startup: we need to bootstrap from whatever local state (including nothing)
    // to the latest network state (or close to latest). until this happen, we don't participate in the network
    // (no block creation) and our network connection(s) is only use to download data.
    //
    // Various aspects to do, similar to hermes:
    // * download all the existing blocks
    // * verify all the downloaded blocks
    // * network / peer discoveries (?)
    // * gclock sync ?

    // Read block state
    // init storage
    // create blockchain storage

    // ** TODO **

    // # Active phase
    //
    // now that we have caught up (or almost caught up) we download blocks from neighbor nodes,
    // listen to announcements and actively listen to synchronous queries
    //
    // There's two simultaenous roles to this:
    // * Leader: decided after global or local evaluation. Need to create and propagate a block
    // * Non-Leader: always. receive (pushed-) blocks from other peers, investigate the correct blockchain updates
    //
    // Also receive synchronous connection queries:
    // * new nodes subscribing to updates (blocks, transactions)
    // * client GetBlocks/Headers ...

    let tpool_data : TPool<TxId, TxAux> = TPool::new();
    let tpool = Arc::new(RwLock::new(tpool_data));

    // ** TODO **
    // setup_network
    //  connection-events:
    //    poll:
    //      recv_transaction:
    //         check_transaction_valid
    //         add transaction to pool
    //      recv_block:
    //         check block valid
    //         try to extend blockchain with block
    //         update utxo state
    //         flush transaction pool if any txid made it
    //      get block(s):
    //         try to answer
    //

    let transaction_task = {
        let tpool = Arc::clone(&tpool);
        task_create_with_inputs("transaction", move |r| transaction_task(tpool, r));
    };
    let block_task = {
        let blockchain = Arc::clone(&blockchain);
        task_create_with_inputs("block", move |r| block_task(blockchain, r));
    };

    let client_task = {
        let blockchain = Arc::clone(&blockchain);
        task_create_with_inputs("client-query", move |r| client_task(blockchain, r));
    };

    //let network_ntt_task = task_create("network", || {
        // listen to native network
        // connect to other nodes
    //});

    let leadership = {
        let tpool = Arc::clone(&tpool);
        task_create("leadership", move || {
            // FIXME this is handled in thread, but the event will come from the clock on new slot event
            let sleep_time = time::Duration::from_secs(20);
            loop {
                thread::sleep(sleep_time);
                let len = {
                    let t = tpool.read().unwrap();
                    (*t).content.len()
                };
                println!("leadership thread waking up (tpool = {} transactions)", len)
                //   check elected
                //   if elected
                //     take set of transactions from pool
                //     create a block
                //     send it async to peers
            }
        })
    };

    // periodically cleanup (custom):
    //   storage cleanup/packing
    //   tpool.gc()

    // FIXME some sort of join so that the main thread does something ...
    println!("Hello, world!");
}
