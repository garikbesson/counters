use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let contract = worker.dev_deploy(&contract_wasm).await?;
 
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

    let _ = user.call(contract.id(), "increment")
        .args_json(json!({}))
        .transact()
        .await?;
    
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
