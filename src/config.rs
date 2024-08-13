use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct NetworkConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub ws_url: String,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
}

pub static NETWORK_CONFIGS: Lazy<HashMap<&'static str, NetworkConfig>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "ethereum",
        NetworkConfig {
            chain_id: 1,
            rpc_url: "https://eth.llamarpc.com".to_string(),
            ws_url: "wss://eth.llamarpc.com".to_string(),
            explorer_url: "https://etherscan.io".to_string(),
            explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
            explorer_api_url: "https://api.etherscan.io/api".to_string(),
        },
    );
    m.insert(
        "arbitrum",
        NetworkConfig {
            chain_id: 42161,
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            ws_url: "wss://arb1.arbitrum.io/ws".to_string(),
            explorer_url: "https://arbiscan.io".to_string(),
            explorer_api_key: "".to_string(),
            explorer_api_url: "https://api.arbiscan.io/api".to_string(),
        },
    );
    m.insert(
        "optimism",
        NetworkConfig {
            chain_id: 10,
            rpc_url: "https://mainnet.optimism.io".to_string(),
            ws_url: "wss://ws-mainnet.optimism.io".to_string(),
            explorer_url: "https://optimistic.etherscan.io".to_string(),
            explorer_api_key: "".to_string(),
            explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
        },
    );
    m
});

pub fn get_network_config(network: &str) -> Option<&'static NetworkConfig> {
    NETWORK_CONFIGS.get(network)
}
