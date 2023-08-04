pub mod acl;
pub mod canister;
pub mod guard;
pub mod storage;

use candid::{Principal, candid_method};
use ic_cdk::{update, query, init, api::management_canister::main::CanisterStatusResponse};
use guard::{is_admin, is_manager, is_viewer};

#[init]
#[candid_method(init)]
fn init() {
    // Add the caller as an admin to bootstrap the canister
    acl::grant_access(ic_cdk::api::caller(), acl::Role::Admin);
}

#[update(guard = "is_admin")]
#[candid_method(update)]
fn grant_access(principal: Principal, role: acl::Role) {
    acl::grant_access(principal, role);
}

#[update(guard = "is_admin")]
#[candid_method(update)]
fn revoke_access(principal: Principal) {
    acl::revoke_access(principal);
}

#[query(guard = "is_admin")]
#[candid_method(query)]
fn get_access_list() -> Vec<acl::Access> {
    acl::get_access_list()
}

#[update(guard = "is_manager")]
#[candid_method(update)]
async fn delegate_call(canister_id: Principal, method: String, args: Vec<u8>) -> Vec<u8> {
    canister::delegate_call(canister_id, method, args).await
}

#[update(guard = "is_viewer")]
#[candid_method(update)]
async fn canister_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    let result = canister::canister_status(canister_id);
    result.await
}
