use std::ffi::c_int;

pub mod request;

unsafe extern "C" {
    fn launch_hello() -> c_int;
}

/// Run the greeting kernel to completion, returning the CUDA status on failure.
pub fn hello_gpu() -> Result<(), c_int> {
    // SAFETY: The launcher takes no pointers and requires no caller-owned state.
    match unsafe { launch_hello() } {
        0 => Ok(()),
        status => Err(status),
    }
}
