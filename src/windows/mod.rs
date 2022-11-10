use std::mem::MaybeUninit;

use winapi::shared::minwindef::FALSE;
use winapi::um::minwinbase::STILL_ACTIVE;
use winapi::um::processthreadsapi::GetExitCodeProcess;

use self::handle::Handle;
use crate::{Pid, State};

mod handle;

pub fn state(pid: Pid) -> State {
    let handle = match Handle::open(pid) {
        Some(handle) => handle,
        None => return State::Unknown,
    };

    let mut status = MaybeUninit::uninit();
    unsafe {
        if GetExitCodeProcess(*handle, status.as_mut_ptr()) == FALSE {
            return State::Unknown;
        }

        if status.assume_init() == STILL_ACTIVE {
            State::Alive
        } else {
            State::Dead
        }
    }
}
