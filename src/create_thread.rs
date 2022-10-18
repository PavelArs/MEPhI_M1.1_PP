extern crate winapi;

// use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::TRUE;
use winapi::um::processthreadsapi::CreateThread;
// use winapi::um::synchapi::CreateMutexW;
use winapi::um::synchapi::WaitForMultipleObjects;
use winapi::um::winbase::INFINITE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::minwindef::DWORD;
use winapi::um::processthreadsapi::GetExitCodeThread;
use winapi::shared::ntdef::NULL;
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::shared::minwindef::LPDWORD;
use crate::my_func;

fn _count_positive_values(lp_parameter: LPVOID) -> DWORD {
    return my_func::count_positive_values(lp_parameter as Vec<f32>) as DWORD;
}

pub fn count_positive_values(arr: Vec<f32>, tn: usize) -> u32 {
    // let mut handles: Vec<HANDLE> = Vec::new();
    let mut handles = vec![];
    // const H_MUTEX: HANDLE = CreateMutexW(NULL, FALSE, NULL);
    // if H_MUTEX == NULL { Err("Failed to create mutex.") };
    for i in 0..tn {
        let l = arr.len();
        let left = num_integer::div_floor(i * l, tn) as usize;
        let right = num_integer::div_floor((i + 1) * l, tn) as usize;
        let handle = CreateThread(
            NULL as LPSECURITY_ATTRIBUTES,
            0,
            _count_positive_values as LPTHREAD_START_ROUTINE,
            (&arr[left..right]).to_vec().as_mut_ptr() as LPVOID,
            0,
            NULL as LPDWORD
        );
        handles.push(handle);
    }
    WaitForMultipleObjects(tn as DWORD, handles.as_mut_ptr(), TRUE, INFINITE);
    let mut counter = 0;
    for i in 0..tn {
        let mut tmp: LPDWORD = 0 as LPDWORD;
        GetExitCodeThread(handles[i], tmp);
        counter += tmp as u32;
        CloseHandle(handles[i]);
    }
    return counter;
}