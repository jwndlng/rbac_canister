use std::cell::RefCell;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use ic_stable_structures::storable::Blob;

// Memory IDs
const _: MemoryId = MemoryId::new(0); // Reserving slot 0 for future use
const AUTH_MEM_ID: MemoryId = MemoryId::new(1);

// Types
type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type AuthMap = StableBTreeMap<Blob<29>, u32, Memory>;

// Stable Storage
thread_local! {
    // init memory manager
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(
            MemoryManager::init(DefaultMemoryImpl::default())
    );
    pub static AUTH: RefCell<AuthMap> =
        MEMORY_MANAGER.with(|mm| {
            RefCell::new(StableBTreeMap::init(mm.borrow().get(AUTH_MEM_ID)))
    });
}


