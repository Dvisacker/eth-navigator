use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifiChain {
    key: String,
    name: String,
    chain_type: String, // renamed from chainType
    coin: String,
    id: u64,
    mainnet: bool,
    logo_uri: String,                 // renamed from logoURI
    tokenlist_url: String,            // renamed from tokenlistUrl
    multicall_address: String,        // renamed from multicallAddress
    faucet_urls: Option<Vec<String>>, // added for optional faucet URLs
    metamask: Metamask,               // new struct for metamask details
    native_token: Token,              // new struct for native token details
}

// New struct for Metamask details
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metamask {
    chain_id: String,                 // renamed from chainId
    block_explorer_urls: Vec<String>, // renamed from blockExplorerUrls
    chain_name: String,               // renamed from chainName
    native_currency: NativeCurrency,  // new struct for native currency details
    rpc_urls: Vec<String>,            // renamed from rpcUrls
}

// New struct for Native Currency details
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    chain_id: u64,
    address: String,
    symbol: String,
    name: String,
    decimals: u8,
    price_usd: String,
    coin_key: String,
    logo_uri: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteRequest {
    from_chain_id: u64,
    to_chain_id: u64,
    from_token_address: String,
    to_token_address: String,
    from_amount: String,
    from_address: String,
    to_address: String,
}

impl RouteRequest {
    pub fn new(
        from_chain_id: u64,
        to_chain_id: u64,
        from_token_address: String,
        to_token_address: String,
        from_amount: String,
        from_address: String,
        to_address: String,
    ) -> Self {
        Self {
            from_chain_id,
            to_chain_id,
            from_token_address,
            to_token_address,
            from_amount,
            from_address,
            to_address,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteResponse {
    pub routes: Vec<Route>,
    pub errors: Option<Vec<RouteError>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub id: String,
    pub from_chain_id: u64,
    pub from_amount_usd: String,
    pub from_amount: String,
    pub from_token: Token,
    pub to_chain_id: u64,
    pub to_amount_usd: String,
    pub to_amount: String,
    pub to_amount_min: String,
    pub to_token: Token,
    pub gas_cost_usd: String,
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
    pub from_token: Token,
    pub to_token: Token,
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
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeCost {
    pub name: String,
    pub description: String,
    pub percentage: String,
    pub token: Token,
    pub amount: String,
    pub amount_usd: String,
    pub included: bool,
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
    pub amount_usd: String,
    pub token: Token,
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

#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "fromChainId")]
    from_chain_id: u64,
    #[serde(rename = "toChainId")]
    to_chain_id: u64,
    #[serde(rename = "fromTokens")]
    from_tokens: Vec<Token>,
    #[serde(rename = "toTokens")]
    to_tokens: Vec<Token>,
}
