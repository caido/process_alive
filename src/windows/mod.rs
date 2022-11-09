use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetExitCodeProcess, OpenProcess};

use self::handle::Handle;
use crate::{Pid, State};

mod handle;

pub fn state(pid: Pid) -> State {
    todo!()
}
