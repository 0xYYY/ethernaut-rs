use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use std::error::Error;
use std::sync::Arc;

abigen!(
    SolutionContract,
    r#"[
        function buy() public
    ]"#,
);

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/21_shop/solution.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let level_address = level.instance.parse::<Address>()?;
    let solution_contract = factory.deploy(level_address)?.legacy().send().await?;
    let solution_address = solution_contract.address();
    println!("Solution contract deployed at {}\n", solution_address);

    let solution_contract = SolutionContract::new(solution_address, client.clone());
    let receipt = solution_contract.buy().legacy().send().await?.await?;
    println!("buy():\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
