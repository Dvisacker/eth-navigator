use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider, Ws},
    signers::Wallet,
    types::Chain,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, env, sync::Arc};

use crate::signer_middleware::setup_signer;

pub struct ChainConfig {
    pub chain: Chain,
    pub chain_id: u64,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub ws: Arc<Provider<Ws>>,
}

// Add this new static mapping
pub static CHAIN_MAP: Lazy<HashMap<String, Chain>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ethereum".to_string(), Chain::Mainnet);
    m.insert("goerli".to_string(), Chain::Goerli);
    m.insert("sepolia".to_string(), Chain::Sepolia);
    m.insert("polygon".to_string(), Chain::Polygon);
    m.insert("mumbai".to_string(), Chain::PolygonMumbai);
    m.insert("arbitrum".to_string(), Chain::Arbitrum);
    m.insert("arbitrum_goerli".to_string(), Chain::ArbitrumGoerli);
    m.insert("optimism".to_string(), Chain::Optimism);
    m.insert("optimism_goerli".to_string(), Chain::OptimismGoerli);
    m
});

pub async fn get_chain_config(chain: Chain) -> ChainConfig {
    match chain {
        Chain::Mainnet => {
            let url = env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL is not set");
            let ws_url = env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set");
            let http_provider = Provider::<Http>::try_from(url).unwrap();
            let middleware = setup_signer(http_provider.clone()).await;
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 1,
                explorer_url: "https://etherscan.io".to_string(),
                explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
                explorer_api_url: "https://api.etherscan.io/api".to_string(),
                http: Arc::new(middleware),
                ws: Arc::new(ws_provider),
            };
        }
        Chain::Arbitrum => {
            let url = env::var("ARBITRUM_RPC_URL").expect("ARBITRUM_RPC_URL is not set");
            let ws_url = env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set");
            let http_provider = Provider::<Http>::try_from(url).unwrap();
            let middleware = setup_signer(http_provider.clone()).await;
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 42161,
                explorer_url: "https://arbiscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.arbiscan.io/api".to_string(),
                http: Arc::new(middleware),
                ws: Arc::new(ws_provider),
            };
        }
        Chain::Optimism => {
            let url = env::var("OPTIMISM_RPC_URL").expect("OPTIMISM_RPC_URL is not set");
            let ws_url = env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set");
            let http_provider = Provider::<Http>::try_from(url).unwrap();
            let middleware = setup_signer(http_provider.clone()).await;
            let ws_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            return ChainConfig {
                chain,
                chain_id: 10,
                explorer_url: "https://optimistic.etherscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
                http: Arc::new(middleware),
                ws: Arc::new(ws_provider),
            };
        }
        _ => panic!("Chain not supported"),
    }
}

// Add this new function to get Chain from string
pub fn get_chain_from_string(chain_name: &str) -> Option<Chain> {
    println!("Chain name: {}", chain_name);
    CHAIN_MAP.get(chain_name).cloned()
}
