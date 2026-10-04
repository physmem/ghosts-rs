use anyhow::Result;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_DELETE};

use log::LevelFilter;
use simple_logger::SimpleLogger;

use windows::Win32::Foundation::{HMODULE, LPARAM, WPARAM};
use windows::Win32::System::Console::{AllocConsole, FreeConsole, GetConsoleWindow};
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageA, WM_CLOSE};

pub static SHOULD_RUN: AtomicBool = AtomicBool::new(true);

mod hooks;
mod iw;
mod managers;
mod utils;

fn on_attach() -> Result<()> {
    unsafe {
        AllocConsole()?;
    }

    SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .with_module_level("ghosts_rs", LevelFilter::Trace)
        .init()?;

    log::info!("Initializing");

    let start_time = Instant::now();

    hooks::setup()?;

    log::info!("Initialized in {:.2}s", start_time.elapsed().as_secs_f32());

    while SHOULD_RUN.load(Ordering::Acquire) {
        unsafe {
            if (GetAsyncKeyState(VK_DELETE.0 as i32) as u16 & 0x8000) != 0 {
                SHOULD_RUN.store(false, Ordering::Relaxed);
            };
        }
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn on_detach() {
    log::info!("Unloading");

    hooks::restore();

    unsafe {
        let console = GetConsoleWindow();

        FreeConsole().ok();

        if !console.is_invalid() {
            PostMessageA(Some(console), WM_CLOSE, WPARAM(0), LPARAM(0)).ok();
        }
    }
}

extern "system" fn thread_start(module: *mut c_void) -> u32 {
    if let Err(e) = on_attach() {
        log::error!("Failed to attach {e}");

        thread::sleep(Duration::from_secs(5));
    }

    on_detach();

    thread::sleep(Duration::from_millis(100));
    unsafe {
        FreeLibraryAndExitThread(HMODULE(module), 1);
    }
}

#[unsafe(export_name = "DllMain")]
extern "system" fn dll_main(
    module: *mut c_void,
    reason_for_call: u32,
    _reversed: *mut c_void,
) -> u32 {
    const DLL_PROCESS_ATTACH: u32 = 1;

    if reason_for_call != DLL_PROCESS_ATTACH {
        return 0;
    }

    unsafe {
        CreateThread(
            None,
            0,
            Some(thread_start),
            Some(module),
            THREAD_CREATION_FLAGS(0),
            None,
        )
        .is_ok() as u32
    }
}
