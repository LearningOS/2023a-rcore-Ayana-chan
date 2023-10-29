//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token,
    }, 
    mm::{translated_byte_buffer, VirtAddr, VirtPageNum}, timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let time_val_size = core::mem::size_of::<TimeVal>();

    let us = get_time_us();
    let ans = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let ans_slice = unsafe{
        core::slice::from_raw_parts(&ans as *const TimeVal as *const u8, time_val_size)
    };

    let aims = translated_byte_buffer(current_user_token(),
     _ts as *const u8, core::mem::size_of::<TimeVal>());
    
    let mut index: usize = 0;
    for _sub in aims{
        for aim_byte in _sub{
            *aim_byte = ans_slice[index];
            index += 1;
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");

    let va_start = VirtAddr::from(_start);

    if !va_start.aligned(){
        return -1;
    }
    if _port & !0x7 != 0 || _port * 0x7 == 0 {
        return -1;
    }
    //有被映射的页 TODO
    //物理内存不足 TODO

    let start_vpn: VirtPageNum = va_start.floor();
    let end_vpn: VirtPageNum = VirtAddr::from(_start + _len).ceil(); // len向上取整
    for _ in usize::from(start_vpn)..=usize::from(end_vpn) {
        
    }

    //frame_alloc请求物理空间，返回None表示没空间了 TODO
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
