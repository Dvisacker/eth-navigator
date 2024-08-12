use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct NetworkConfig {
    pub rpc_url: String,
    pub ws_url: String,
}

pub static NETWORK_CONFIGS: Lazy<HashMap<&'static str, NetworkConfig>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "ethereum",
        NetworkConfig {
            rpc_url: "https://eth.llamarpc.com".to_string(),
            ws_url: "wss://eth.llamarpc.com".to_string(),
        },
    );
    m.insert(
        "arbitrum",
        NetworkConfig {
            rpc_url: "https://arb1.arbitrum.io/rpc".to_string(),
            ws_url: "wss://arb1.arbitrum.io/ws".to_string(),
        },
    );
    m.insert(
        "optimism",
        NetworkConfig {
            rpc_url: "https://mainnet.optimism.io".to_string(),
            ws_url: "wss://ws-mainnet.optimism.io".to_string(),
        },
    );
    m
});

pub fn get_network_config(network: &str) -> Option<&'static NetworkConfig> {
    NETWORK_CONFIGS.get(network)
}
