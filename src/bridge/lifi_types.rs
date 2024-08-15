use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiAPIChainResponse {
    pub chains: Vec<LifiChain>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiTokenListResponse {
    pub tokens: HashMap<String, Vec<LifiToken>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiConnectionResponse {
    pub connections: Vec<LifiConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiChain {
    pub key: String,
    pub name: String,
    pub chain_type: String, // renamed from chainType
    pub coin: String,
    pub id: u64,
    pub mainnet: bool,
    pub logo_URI: String,                 // renamed from logoURI
    pub tokenlist_url: Option<String>,    // renamed from tokenlistUrl
    pub multicall_address: String,        // renamed from multicallAddress
    pub faucet_urls: Option<Vec<String>>, // added for optional faucet URLs
    pub metamask: Metamask,               // new struct for metamask details
    pub native_token: LifiToken,          // new struct for native token details
}

// New struct for Metamask details
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metamask {
    pub chain_id: String,                 // renamed from chainId
    pub block_explorer_urls: Vec<String>, // renamed from blockExplorerUrls
    pub chain_name: String,               // renamed from chainName
    pub native_currency: NativeCurrency,  // new struct for native currency details
    pub rpc_urls: Vec<String>,            // renamed from rpcUrls
}

// New struct for Native Currency details
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LifiToken {
    pub chain_id: u64,
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub price_USD: String,
    pub coin_key: Option<String>,
    pub logo_URI: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteRequest {
    from_chain_id: u64,
    to_chain_id: u64,
    from_token_address: String,
    to_token_address: String,
    from_amount: String,
    // from_address: String,
    // to_address: String,
}

impl RouteRequest {
    pub fn new(
        from_chain_id: u64,
        to_chain_id: u64,
        from_token_address: String,
        to_token_address: String,
        from_amount: String,
        // from_address: String,
        // to_address: String,
    ) -> Self {
        Self {
            from_chain_id,
            to_chain_id,
            from_token_address,
            to_token_address,
            from_amount,
            // from_address,
            // to_address,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiRouteResponse {
    pub routes: Vec<LifiRoute>,
    pub errors: Option<Vec<RouteError>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiRoute {
    pub id: String,
    pub from_chain_id: u64,
    pub from_amount_usd: Option<String>,
    pub from_amount: String,
    pub from_token: LifiToken,
    pub to_chain_id: u64,
    pub to_amount_usd: Option<String>,
    pub to_amount: String,
    pub to_amount_min: String,
    pub to_token: LifiToken,
    pub gas_cost_usd: Option<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    #[serde(rename = "type")]
    pub step_type: String,
    pub tool: String,
    pub action: Action,
    pub estimate: Estimate,
    pub integrator: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub from_token: LifiToken,
    pub to_token: LifiToken,
    pub from_amount: String,
    pub slippage: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Estimate {
    pub from_amount: String,
    pub to_amount: String,
    pub to_amount_min: String,
    pub approval_address: String,
    pub fee_costs: Vec<FeeCost>,
    pub gas_costs: Vec<GasCost>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeCost {
    pub name: String,
    pub description: String,
    pub percentage: String,
    pub token: LifiToken,
    pub amount: String,
    pub amount_usd: Option<String>,
    pub included: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCost {
    #[serde(rename = "type")]
    pub cost_type: String,
    pub price: String,
    pub estimate: String,
    pub limit: String,
    pub amount: String,
    pub amount_usd: Option<String>,
    pub token: LifiToken,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteError {
    pub error_type: String,
    pub code: String,
    pub action: Action,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    from_chain: String,
    to_chain: String,
    from_token: String,
    to_token: String,
    from_amount: String,
    from_address: String,
    to_address: String,
}

impl QuoteRequest {
    pub fn new(
        from_chain: String,
        to_chain: String,
        from_token: String,
        to_token: String,
        from_amount: String,
        from_address: String,
        to_address: String,
    ) -> Self {
        Self {
            from_chain,
            to_chain,
            from_token,
            to_token,
            from_amount,
            from_address,
            to_address,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Quote {
    id: String,
    // Add other fields as needed
}

// Add this new struct to the existing ones
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusRequest {
    pub bridge: Option<String>,
    pub from_chain: Option<String>,
    pub to_chain: Option<String>,
    pub tx_hash: String,
}

impl StatusRequest {
    pub fn new(
        bridge: Option<String>,
        from_chain: Option<String>,
        to_chain: Option<String>,
        tx_hash: String,
    ) -> Self {
        Self {
            bridge,
            from_chain,
            to_chain,
            tx_hash,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    status: String,
    // Add other fields as needed
}

// Add these new structs
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsRequest {
    pub from_chain: Option<String>,
    pub to_chain: Option<String>,
    pub from_token: Option<String>,
    pub to_token: Option<String>,
    pub from_amount: Option<String>,
    pub allow_exchanges: Option<bool>,
}

impl ConnectionsRequest {
    pub fn new(
        from_chain: Option<String>,
        to_chain: Option<String>,
        from_token: Option<String>,
        to_token: Option<String>,
        from_amount: Option<String>,
        allow_exchanges: Option<bool>,
    ) -> Self {
        Self {
            from_chain,
            to_chain,
            from_token,
            to_token,
            from_amount,
            allow_exchanges,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiConnection {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub from_tokens: Vec<LifiToken>,
    pub to_tokens: Vec<LifiToken>,
}
