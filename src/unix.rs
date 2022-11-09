use libc::kill;

use crate::{Pid, State};

pub fn state(pid: Pid) -> State {
    let pid: i32 = match pid.0.try_into() {
        Ok(pid) => pid,
        Err(_) => return State::Unknown,
    };

    unsafe {
        if kill(pid, 0) == 0 {
            return State::Alive;
        }

        let errno = libc::__error();
        if !errno.is_null() && *errno == libc::ESRCH {
            State::Dead
        } else {
            State::Unknown
        }
    }
}
