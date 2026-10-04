use std::ffi::{CString, c_char};
use std::mem;

use crate::managers::PatternManager;

#[repr(C)]
#[derive(Copy, Clone)]
pub union DvarValue {
    pub bool_value: bool,
    pub int_value: i32,
    pub dw_value: u32,
    pub fl_value: f32,
    pub sz_value: *mut c_char,
    pub vec4_value: [f32; 4],
}

#[repr(C)]
pub struct Dvar {
    pub name: *mut c_char,
    pub flags: u32,
    pub dvar_type: i8,
    pub modified: bool,
    pub pad: [u8; 2],
    pub current: DvarValue,
    pub latched: DvarValue,
    pub reset: DvarValue,
    _mem_pad: [u8; 0x20],
}

pub type FindDvarFn = unsafe extern "C" fn(*mut c_char) -> *mut Dvar;

pub fn find_dvar(dvar_name: &str) -> *mut Dvar {
    let name = CString::new(dvar_name).unwrap();

    let address = PatternManager::instance().address("Dvar_FindVar").unwrap();
    let find_dvar: FindDvarFn = unsafe { mem::transmute(address.get()) };

    unsafe { find_dvar(name.as_ptr().cast_mut()) }
}
