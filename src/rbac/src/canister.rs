use ic_cdk::api::{
    management_canister::main::{
        CanisterIdRecord, CanisterStatusResponse, InstallCodeArgument,
        CanisterInstallMode
    }
};
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

pub async fn install_code(
    canister_id: Principal, mode: CanisterInstallMode, wasm_code: Vec<u8>, arg: Vec<u8>
) -> Result<(), String> {
    let install_code_argument = InstallCodeArgument {
        mode: mode,
        canister_id: canister_id,
        wasm_module: wasm_code,
        arg: arg,
    };

    let call_result = ic_cdk::api::management_canister::main::install_code(install_code_argument).await;
    match call_result {
        Ok(result) => Ok(result),
        Err(_) => Err("Code cannot be installed.".to_string())
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
