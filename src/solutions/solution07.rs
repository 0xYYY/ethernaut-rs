use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    SolutionContract,
    r#"[
        function solve() public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/07_force/solution.sol")).await?;
    let contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        contract.abi.clone(),
        contract.bytecode.clone(),
        client.clone(),
    );

    println!("Deploying solution contract");
    let mut deployer = factory.deploy(level.instance.parse::<Address>()?)?.legacy();
    deployer.tx.set_value(1);
    let contract = deployer.send().await?;
    let addr = contract.address();
    println!("Solution contract deployed at {}\n", addr);

    let contract = SolutionContract::new(addr, client.clone());
    let receipt = contract.solve().legacy().send().await?.await?;
    println!("solve():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
