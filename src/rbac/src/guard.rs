use crate::storage::AUTH;
use ic_stable_structures::{storable::Blob as StorableBlob, Storable};
use crate::acl::Role::SuperAdmin;

pub fn is_authorized_admin() -> Result<(), String> {
    AUTH.with(|a| {
        if let Some(value) = a.borrow().get(&StorableBlob::from_bytes(
            ic_cdk::api::caller().as_slice().into(),
        )) {
            if value <= SuperAdmin as u32 {
                Ok(())
            } else {
                Err("is_authorized_admin(): You are not authorized.".to_string())
            }
        } else {
            Err("is_authorized_admin(): You are not authorized.".to_string())
        }
    })
}

// canister admin access
pub fn is_authorized() -> Result<(), String> {
    AUTH.with(|a| {
        if a.borrow().contains_key(&StorableBlob::from_bytes(
            ic_cdk::api::caller().as_slice().into(),
        )) {
            Ok(())
        } else {
            Err("is_authorized_user(): You are not authorized.".to_string())
        }
    })
}
