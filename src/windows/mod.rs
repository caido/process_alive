use std::mem::MaybeUninit;

use winapi::shared::minwindef::LPDWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::STILL_ACTIVE;
use winapi::um::processthreadsapi::{GetExitCodeProcess, OpenProcess};

use self::handle::Handle;
use crate::{Pid, State};

mod handle;

pub fn state(pid: Pid) -> State {
    let handle = match Handle::open(pid) {
        Some(handle) => handle,
        None => State::Unknown,
    };

    let mut status = MaybeUninit::uninit();
    unsafe {
        if !GetExitCodeProcess(*handle, status.as_mut_ptr()) {
            return Status::Unknown;
        }

        if status.assume_init() == STILL_ACTIVE {
            Status::Alive
        } else {
            Status::Dead
        }
    }
}
