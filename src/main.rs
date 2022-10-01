// tx data:
// 0xe70dd2fc000000000000000000000000000000000000000000000000000000000000014000000000000000000000000082af49447d8a07e3bd95bd0d56f35241523fbab10000000000000000000000000000000000000005ac103444df194d6e77b0393e000000000000000000000000000000000000000b5c0e8d21d902d61fa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082828f6aff831e0d8b366d7b33caf12b3923277200000000000000000000000000000000000041b6e3927bcdbbf5fa7c74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005af3107a400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000ff970a61a04b1ca14834a43f5de4533ebddb5cc8

use std::sync::Arc;

use ethers::{
    abi::{AbiDecode, decode},
    prelude::{abigen, k256::ecdsa::SigningKey, rand::thread_rng, Abigen, SignerMiddleware, *},
    providers::{Http, Provider},
};

pub const contract_address: &str = "0x3D6bA331e3D9702C5e8A8d254e5d8a285F223aba";
abigen!(SmartContract, "src/abi.json");

pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

pub fn bind(name: &str, abi: &str) {
    let name: String = format!("b_{}", name);
    let bindings = Abigen::new(&name, abi).unwrap().generate().unwrap();
    let path: String = format!("src/{}.rs", name);
    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(_) => {}
    }
    bindings.write_to_file(&path).unwrap();
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    network: String,
    provider: Provider<Http>,
    pub middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Config {
    pub fn new(network: &str, chain_id: &u64) -> Self {
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let wallet = LocalWallet::new(&mut thread_rng()).with_chain_id(*chain_id);
        let middleware = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

        Self {
            network: network.to_string(),
            provider: provider,
            middleware: middleware,
        }
    }

    pub fn create_contract(
        &self,
    ) -> SmartContract<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
        bind("contract", "src/abi.json");
        let addr: H160 = address(contract_address);
        SmartContract::new(addr, Arc::clone(&self.middleware))
    }
}

fn main() {
    // println!("////////// Setting up env //////////");
    // let config = Config::new("https://arb1.arbitrum.io/rpc", &42161);
    // let contract_middleware = config.create_contract();

    println!("////////// Decoding Tx //////////");
    let tx_data = "0xe70dd2fc000000000000000000000000000000000000000000000000000000000000014000000000000000000000000082af49447d8a07e3bd95bd0d56f35241523fbab10000000000000000000000000000000000000005ac103444df194d6e77b0393e000000000000000000000000000000000000000b5c0e8d21d902d61fa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082828f6aff831e0d8b366d7b33caf12b3923277200000000000000000000000000000000000041b6e3927bcdbbf5fa7c74000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005af3107a400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000ff970a61a04b1ca14834a43f5de4533ebddb5cc8";
    let calldata: Bytes = tx_data.parse().unwrap();
    let decoded = SmartContractCalls::decode(&calldata).unwrap();
    println!("{:?}", decoded);
}
