use std::ffi::c_void;

use retour::static_detour;

pub type IsItemUnlockedFn = unsafe extern "C" fn(*const c_void, *const c_void, u32) -> bool;

static_detour! {
    pub static IsItemUnlockedHook: unsafe extern "C" fn(
        *const c_void,
        *const c_void,
        u32
    ) -> bool;
}

pub fn hooked_is_item_unlocked(
    _a1: *const c_void,
    _csv_data: *const c_void,
    _row_index: u32,
) -> bool {
    true

    // uncomment this out if you dont want to unlock everything
    //unsafe { IsItemUnlockedHook.call(a1, csv_data, row_index) }
}
