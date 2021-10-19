use ethers_middleware::signer::SignerMiddleware;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{coins_bip39::English, MnemonicBuilder, Signer};
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

pub fn create_signer_middleware(
    wallet_mnemonic_path: String,
    chain_id: u64,
    rpc: String,
) -> Result<impl Middleware, Box<dyn Error>> {
    let mnemonic = fs::read_to_string(wallet_mnemonic_path)?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str().trim())
        .build()?
        .with_chain_id(chain_id);

    let provider = Provider::<Http>::try_from(rpc)?;
    let client = SignerMiddleware::new(provider, wallet);

    Ok(client)
}
