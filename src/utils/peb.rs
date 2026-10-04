use std::arch::asm;
use std::mem::offset_of;
use std::slice;
use std::sync::{LazyLock, OnceLock};

use windows::Win32::System::Threading::PEB;
use windows::Win32::System::WindowsProgramming::LDR_DATA_TABLE_ENTRY;

use super::{Address, Module};

macro_rules! containing_record {
    ($addr:expr, $type:path, $field:ident) => {
        ($addr as usize - offset_of!($type, $field)) as *mut $type
    };
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Peb(PEB);

impl Peb {
    pub fn new() -> Self {
        let peb = unsafe { *(__readgsqword(0x60) as *const PEB) };

        Self(peb)
    }

    #[inline]
    pub fn instance() -> &'static Self {
        static INSTANCE: LazyLock<Peb> = LazyLock::new(Peb::new);

        &INSTANCE
    }

    #[inline]
    pub fn main_module(&self) -> Option<&'static Module> {
        self.modules().first()
    }

    #[inline]
    pub fn module(&self, name: &str) -> Option<&'static Module> {
        self.modules()
            .iter()
            .find(|module| module.name.eq_ignore_ascii_case(name))
    }

    pub fn modules(&self) -> &'static [Module] {
        static MODULES: OnceLock<Vec<Module>> = OnceLock::new();

        MODULES.get_or_init(|| {
            let list = unsafe { &mut (*self.0.Ldr).InMemoryOrderModuleList };

            let mut link = list.Flink;
            let mut modules = Vec::new();

            while link != list {
                unsafe {
                    let entry =
                        &*containing_record!(link, LDR_DATA_TABLE_ENTRY, InMemoryOrderLinks);

                    let path = String::from_utf16_lossy(slice::from_raw_parts(
                        entry.FullDllName.Buffer.as_ptr(),
                        entry.FullDllName.Length as usize / 2,
                    ));

                    let name = path.rsplit('\\').next().unwrap_or(path.as_str());

                    modules.push(Module::new(
                        name,
                        Address::new(entry.DllBase as usize),
                        entry.Reserved3[1] as usize,
                    ));

                    link = (*link).Flink;
                }
            }

            modules
        })
    }
}

unsafe impl Send for Peb {}
unsafe impl Sync for Peb {}

#[inline(always)]
fn __readgsqword(offset: u64) -> u64 {
    let mut out: u64;

    unsafe {
        asm!(
            "mov {}, gs:[{}]",
            out(reg) out,
            in(reg) offset,
            options(nostack, preserves_flags, readonly)
        );
    }

    out
}
