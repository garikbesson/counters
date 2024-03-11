use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;
 
const CONTRACT_WASM_FILEPATH: &str = "../target/wasm32-unknown-unknown/release/contract.wasm";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(CONTRACT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;
 
    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;
 
    // begin tests
    test_can_be_incremented(&alice, &contract).await?;

    Ok(())
}
 
async fn test_can_be_incremented(
    user: &Account,
    contract: &Contract,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_counter: u8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;

    user.call(contract.id(), "increment")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;
    
    let end_counter: u8 = user
        .call(contract.id(), "get_num")
        .args_json(json!({}))
        .transact()
        .await?
        .json()?;
 
    assert_eq!(end_counter, start_counter + 1);
    println!("      Passed ✅ test_can_be_incremented");
    Ok(())
}
