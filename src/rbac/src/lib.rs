
// Role based access control canister

/*

This canister will have two access control mechanism:
1. Access control for the canister itself
2. Access control for other canisters

1. Access control for the canister itself
The idea of this canister is to provide rbac functionality for the canister itself.
The roles will be predefined and the canister will provide the following functions:
- superadmin can do everything
- manager can add new canisters and assign roles to users

2. Access control for other canisters
The idea of this canister is to provide rbac functionality for other canisters.
The roles will be predefined and the canister will provide the following functions:
- superadmin can do everything on another canister
- manager can fully manage a canister but not change its controller
- CI can only deploy a canister
- Read only can only read from a canister like its status for cycle monitoring

*/
use ic_cdk::{
    init,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
};
use std::clone::Clone;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
enum Role {
    Admin,
    Manager,
    Deploy,
    ReadOnly,
}

#[derive(Clone, Debug, Deserialize)]
struct UserAccess {
    pub principal: Principal,
    pub description: String,
    pub role: Role,
}

type InternalMapping = HashMap<Principal, UserAccess>;
type ExternalMapping = HashMap<Principal, UserAccess>;

thread_local! {
    static INTERNAL_MAPPING: RefCell<InternalMapping> = RefCell::default();
    static EXTERNAL_MAPPING: RefCell<ExternalMapping> = RefCell::default();
}

#[init]
fn init() {
    // pass
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
