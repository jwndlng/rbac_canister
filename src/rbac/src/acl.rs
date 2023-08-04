use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::Storable;
use ic_stable_structures::storable::Blob as StorableBlob;
use num_derive::FromPrimitive;
use num::FromPrimitive;
use crate::storage::AUTH;


#[derive(Clone, Debug, CandidType, Deserialize, FromPrimitive)]
pub enum Role {
    Admin,
    Manager,
    Viewer,
    Anonymous,
}

impl Role {

    pub fn has_permission(&self, required_role: Role) -> bool {
        match required_role {
            Role::Admin => self.is_admin(),
            Role::Manager => self.is_manager(),
            Role::Viewer => self.is_viewer(),
            _ => false
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            Role::Admin => true,
            _ => false
        }
    }
    pub fn is_manager(&self) -> bool {
        match self {
            Role::Admin => true,
            Role::Manager => true,
            _ => false
        }
    }
    pub fn is_viewer(&self) -> bool {
        match self {
            Role::Admin => true,
            Role::Manager => true,
            Role::Viewer => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Access {
    id: Principal,
    role: Role,
}

pub fn revoke_access(principal: Principal) {
    AUTH.with(|a| {
        a.borrow_mut()
            .remove(&StorableBlob::from_bytes(principal.as_slice().into()))
            .unwrap();
    });
}

pub fn grant_access(principal: Principal, value: Role) {
    AUTH.with(|a| {
        a.borrow_mut()
            .insert(
                StorableBlob::from_bytes(principal.as_slice().into()),
                value as u32,
            );
    });
}

pub fn get_access_list() -> Vec<Access> {
    let mut access_list = Vec::<Access>::new();
    AUTH.with(|a| {
        for (k, v) in a.borrow().iter() {
            let role: Role = Role::from_u32(v as u32).unwrap();
            access_list.push(Access {
                id: Principal::from_slice(&k.to_bytes()),
                role: role
            });
        }
    });
    access_list
}
