mod acl;
mod storage;
mod guard;

use candid::{Principal, candid_method};
use ic_cdk::{update, query, init};
use guard::is_authorized_admin;


#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::println!("rbac::init() - caller: {:?}", ic_cdk::api::caller());
    // Add the caller as an admin to bootstrap the canister
    acl::authorize(ic_cdk::api::caller(), acl::Role::SuperAdmin);
}

// Local access permissions
#[update(guard = "is_authorized_admin")]
#[candid_method(update)]
fn authorize(principal: Principal, role: acl::Role) {
    acl::authorize(principal, role);
}

#[update(guard = "is_authorized_admin")]
#[candid_method(update)]
fn deauthorize(principal: Principal) {
    acl::deauthorize(principal);
}

#[query(guard = "is_authorized_admin")]
#[candid_method(query)]
fn get_authorized() -> Vec<acl::Access> {
    acl::get_authorized()
}
