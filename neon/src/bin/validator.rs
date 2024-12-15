pub mod validator_essentials;

use globals::Globals;
use neon::{genesis_block::get_genesis_block, history::generate_slot};
use rocket::{fairing::AdHoc, Config};
use std::sync::LazyLock;
use neon::block::Block;
#[macro_use] extern crate rocket;

mod globals {
    use crypto::KeyPair;
    use neon::blockchain::Blockchain;
    use bip39::Mnemonic;
    use std::sync::{Mutex, RwLock};
    use std::fs;

    use crate::validator_essentials::validator_config::ValidatorConfig;

    pub struct Globals {
        pub blockchain: Mutex<Blockchain>,
        pub other_nodes: Vec<String>,
        pub keypair: RwLock<KeyPair>
    }

    impl Globals {
        pub fn new() -> Self {
            let config_string = fs::read_to_string("validator_config.yaml").expect("Fatal error: failed loading config; file not found");
            let config: ValidatorConfig = serde_yaml::from_str(&config_string).expect("Fatal error: failed loading config; invalid yaml schema");

            Self {
                blockchain: Mutex::new(Blockchain::init()),
                other_nodes: Vec::new(),
                keypair: RwLock::new(KeyPair::from_mnemonic(&Mnemonic::parse(config.seed_phrase).expect("Fatal error: user provided an invalid seed phrase")))
            }
        }
    }
}

#[allow(non_upper_case_globals)]
static mut globals: LazyLock<Globals> = LazyLock::new(|| globals::Globals::new());

#[launch]
fn rocket() -> _ {
    unsafe {
        validator_main();
    }

    rocket::build()
        .configure(
            Config { port: 9070, ..Config::debug_default() }
        )
        .mount("/", routes![])
}

unsafe fn validator_main() {
    loop {
        // Get latest block
        let blockchain = globals.blockchain.lock().unwrap();
        let latest_block = blockchain.get_latest_block();
        drop(blockchain);
        
        let latest_hash = latest_block.inner.history[latest_block.inner.history.len()-1].hash.clone();
        
        // Create new block
        let new_history = generate_slot(latest_hash.clone());
        let new_block = Block::new(new_history, latest_block.inner.height+1, &mut globals.keypair.write().unwrap());

        println!("Height: {}; Hash: {}", &new_block.inner.height, latest_hash);
        println!("asdasdsadasdasdasd");
        // ========================================================

        globals.blockchain.lock().unwrap().blocks.push(new_block);
    }
}
