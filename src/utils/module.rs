use std::slice;

use pelite::pattern::Atom;
use pelite::pe64::exports::Export as PeExport;
use pelite::pe64::{Pe, PeView, Rva};

use crate::utils::Address;

#[derive(Clone, Debug)]
pub struct Export {
    pub name: String,
    pub address: Address,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: String,
    pub base: Address,
    pub size: usize,
}

impl Module {
    pub fn new(name: impl Into<String>, base: Address, size: usize) -> Self {
        Self {
            name: name.into(),
            base,
            size,
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.base.as_ptr::<u8>(), self.size) }
    }

    pub fn find_export(&self, name: &str) -> Option<Export> {
        let PeExport::Symbol(rva) = self.view()?.exports().ok()?.by().ok()?.name(name).ok()? else {
            return None;
        };

        Some(Export {
            name: name.to_owned(),
            address: self.base.add(*rva as usize),
        })
    }

    pub fn find_pattern(&self, pattern: &[Atom], rvas: &mut [Rva]) -> bool {
        self.view()
            .is_some_and(|view| view.scanner().finds_code(pattern, rvas))
    }

    #[inline]
    fn view(&self) -> Option<PeView<'_>> {
        PeView::from_bytes(self.as_bytes()).ok()
    }
}
