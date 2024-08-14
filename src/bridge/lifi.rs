use reqwest::Client;
use std::error::Error;

const LIFI_API_URL: &str = "https://li.quest/v1";

use crate::bridge::lifi_types::*;

pub struct LiFiBridge {
    client: Client,
}

impl LiFiBridge {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_supported_chains(&self) -> Result<Vec<LifiChain>, Box<dyn Error>> {
        let url = format!("{}/chains", LIFI_API_URL);
        let response = self.client.get(&url).send().await?;
        let chains: Vec<LifiChain> = response.json().await?;
        Ok(chains)
    }

    pub async fn get_known_tokens(&self, chain: &str) -> Result<Vec<Token>, Box<dyn Error>> {
        let url = format!("{}/tokens?chain={}", LIFI_API_URL, chain);
        let response = self.client.get(&url).send().await?;
        let tokens: Vec<Token> = response.json().await?;
        Ok(tokens)
    }

    pub async fn request_routes(
        &self,
        request: RouteRequest,
    ) -> Result<Vec<Route>, Box<dyn Error>> {
        let url = format!("{}/advanced/routes", LIFI_API_URL);
        let response = self.client.post(&url).json(&request).send().await?;
        let routes: Vec<Route> = response.json().await?;
        Ok(routes)
    }

    pub async fn request_quote(&self, request: QuoteRequest) -> Result<Quote, Box<dyn Error>> {
        let url = format!("{}/quote", LIFI_API_URL);
        let response = self.client.get(&url).query(&request).send().await?;
        let quote: Quote = response.json().await?;
        Ok(quote)
    }

    pub async fn transfer_tokens(&self, quote: Quote) -> Result<String, Box<dyn Error>> {
        // Note: This is a placeholder implementation. In a real-world scenario,
        // you would need to sign the transaction and send it to the blockchain.
        // This typically involves using a wallet or signer, which is beyond
        // the scope of this example.

        println!("Transferring tokens using quote: {:?}", quote);
        Ok("Transaction hash would be returned here".to_string())
    }

    // Add this new method to the existing impl block
    pub async fn get_transfer_status(
        &self,
        request: StatusRequest,
    ) -> Result<StatusResponse, Box<dyn Error>> {
        let url = format!("{}/status", LIFI_API_URL);
        let response = self.client.get(&url).query(&request).send().await?;
        let status: StatusResponse = response.json().await?;
        Ok(status)
    }

    // Add this new method to the existing impl block
    pub async fn get_connections(
        &self,
        request: ConnectionsRequest,
    ) -> Result<Vec<Connection>, Box<dyn Error>> {
        let url = format!("{}/connections", LIFI_API_URL);
        let response = self.client.get(&url).query(&request).send().await?;
        let connections: Vec<Connection> = response.json().await?;
        Ok(connections)
    }
}
