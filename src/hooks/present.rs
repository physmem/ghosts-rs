use crate::iw::find_dvar;

use retour::static_detour;

use windows::Win32::Graphics::Dxgi::{DXGI_PRESENT, IDXGISwapChain};
use windows::core::HRESULT;

pub type PresentFn = unsafe extern "system" fn(IDXGISwapChain, u32, DXGI_PRESENT) -> HRESULT;

static_detour! {
    pub static PresentHook: unsafe extern "system" fn(
        IDXGISwapChain,
        u32,
        DXGI_PRESENT,
    ) -> HRESULT;
}

pub fn hooked_present(
    swap_chain: IDXGISwapChain,
    sync_interval: u32,
    flags: DXGI_PRESENT,
) -> HRESULT {
    unsafe {
        let com_maxfps = find_dvar("com_maxfps");

        if !com_maxfps.is_null() {
            (*com_maxfps).current.fl_value = 1000.0;
        }

        PresentHook.call(swap_chain, sync_interval, flags)
    }
}
