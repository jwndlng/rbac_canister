use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::Storable;
use ic_stable_structures::storable::Blob as StorableBlob;
use num_derive::FromPrimitive;
use num::FromPrimitive;
use crate::storage::AUTH;


#[derive(Clone, Debug, CandidType, Deserialize, FromPrimitive)]
pub enum Role {
    SuperAdmin,
    Admin,
    Manager,
    Viewer
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Access {
    id: Principal,
    role: Role,
}

pub fn deauthorize(principal: Principal) {
    AUTH.with(|a| {
        a.borrow_mut()
            .remove(&StorableBlob::from_bytes(principal.as_slice().into()))
            .unwrap();
    });
}

pub fn authorize(principal: Principal, value: Role) {
    ic_cdk::println!("rbac::authorize() - principal: {:?}, value: {:?}", principal, value);
    AUTH.with(|a| {
        a.borrow_mut()
            .insert(
                StorableBlob::from_bytes(principal.as_slice().into()),
                value as u32,
            );
    });
}

pub fn get_authorized() -> Vec<Access> {
    let mut authorized = Vec::<Access>::new();
    AUTH.with(|a| {
        for (k, v) in a.borrow().iter() {
            if let Some(role) = Role::from_u32(v as u32) {
                authorized.push(Access {
                    id: Principal::from_slice(&k.to_bytes()),
                    role: role
                });
            }
        }
    });
    authorized
}

pub fn permission_mapping(method: &str) -> Role {
    match method {
        "add_admin" => Role::SuperAdmin,                    // Internal call
        "remove_admin" => Role::SuperAdmin,                 // Internal call
        "get_admins" => Role::SuperAdmin,                   // Internal call
        "canister_update_settings" => Role::SuperAdmin,     // IC Management call
        "canister_status" => Role::Viewer,                  // IC Management call
        "install_code" => Role::Manager,                    // IC Management call
        _ => Role::Admin,                                   // Any other canister call
    }
}
