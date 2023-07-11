
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
    api::call::ManualReply, export::Principal, init, post_upgrade, pre_upgrade, query, storage,
    update,
};
use std::collections::{BTreeMap, BTreeSet};

enum AccessRole {
    Admin,
    Manager
}

enum CanisterRole {
    Admin,
    Manager,
    Deploy,
    ReadOnly,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct CanisterAccess {
    pub principal: Principal,
    pub description: String,
    pub keywords: Vec<String>,
}

type Roles = BTreeSet<Principal, AccessRole>;
type CanisterRoles = BTreeMap<String, CanisterAccess>;

thread_local! {
    static ROLES: RefCell<Roles> = RefCell::default();
    static CANISTER_ROLES: RefCell<CanisterRoles> = RefCell::default();
}

#[init]
fn init() {
    // pass
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
