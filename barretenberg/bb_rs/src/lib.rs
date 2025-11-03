use std::ffi::{c_char, CStr};
use tracing::debug;

pub mod barretenberg_api;

#[no_mangle]
extern "C" fn env_hardware_concurrency() -> u32 {
    std::thread::available_parallelism()
        .map(|nz| nz.get().min(u32::MAX as usize) as u32)
        .unwrap_or(1)
}

#[no_mangle]
extern "C" fn logstr(char_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    debug!("{}", c_str.to_str().unwrap());
}

#[no_mangle]
extern "C" fn throw_or_abort_impl(err: *const c_char) -> ! {
    let message = if err.is_null() {
        "barretenberg requested abort without message".to_owned()
    } else {
        unsafe { CStr::from_ptr(err) }.to_string_lossy().into_owned()
    };

    panic!("{message}");
}
