use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterStatusResponse};
use candid::Principal;

pub async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    let status_result = ic_cdk::api::management_canister::main::canister_status(CanisterIdRecord {
        canister_id: canister_id,
    })
    .await;
    match status_result {
        Ok(status) => Ok(status.0),
        Err(_) => Err("Canister status call failed.".to_string())
    }
}

pub async fn delegate_call(canister_id: Principal, method: String, args: Vec<u8>) -> Vec<u8> {
    let response = ic_cdk::api::call::call_raw(
        canister_id,
        &method.as_str(),
        args,
        0,
    )
    .await
    .unwrap();
    response
}
