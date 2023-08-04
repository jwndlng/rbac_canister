use crate::storage::AUTH;
use ic_stable_structures::{storable::Blob as StorableBlob, Storable};
use crate::acl::Role;
use num::FromPrimitive;

pub fn is_admin() -> Result<(), String> {
    is_authorized(Role::Admin)
}

pub fn is_manager() -> Result<(), String> {
    is_authorized(Role::Manager)
}

pub fn is_viewer() -> Result<(), String> {
    is_authorized(Role::Viewer)
}

pub fn is_authorized(required_role: Role) -> Result<(), String> {
    AUTH.with(|a| {
        let role: Role = Role::from_u32(
            a.borrow().get(&StorableBlob::from_bytes(
                ic_cdk::api::caller().as_slice().into()
            )).unwrap_or(Role::Anonymous as u32)
        ).unwrap();
        if role.has_permission(required_role) {
            return Ok(());
        }
        return Err("You are not authorized.".to_string());
    })
}