use crate::load_or_create_whitelist;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
};

pub async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let whitelist = load_or_create_whitelist().expect("Failed to load or create whitelist");
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id.");

    let priv_key = std::env::var("DEV_PRIVATE_KEY").expect("missing PRIVATE_KEY");

    let wallet = priv_key
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());

    if !whitelist.is_wallet_whitelisted(&wallet.address().to_string()) {
        panic!("Wallet is not whitelisted");
    }

    let signer = SignerMiddleware::new(provider, wallet);
    return signer;
}
