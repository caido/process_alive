use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_LIMITED_INFORMATION};

use crate::Pid;

pub struct Handle(HANDLE);

impl Handle {
    fn open(pid: Pid) -> Option<Self> {
        let handle =
            unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid.0 as DWORD) };
        if handle.is_null() {
            None
        } else {
            Some(Self(handle))
        }
    }
}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
