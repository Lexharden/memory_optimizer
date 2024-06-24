extern crate winapi;

use winapi::um::psapi::EmptyWorkingSet;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::winnt::HANDLE;

pub fn free_memory() {
    unsafe {
        let process: HANDLE = GetCurrentProcess();
        EmptyWorkingSet(process);
    }
}
