//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{fetch_curr_task_control_block, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::{get_time_us, get_time},
    sync::UPSafeCell
};
use lazy_static::*;

lazy_static! {
    pub static ref COUNT_SYSCALL_TIMES: UPSafeCell<[u32; MAX_SYSCALL_NUM]> = unsafe{UPSafeCell::new([0; MAX_SYSCALL_NUM])};
}

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
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let curr_task_cb = fetch_curr_task_control_block();
    unsafe {
        (*_ti).status = (*curr_task_cb).task_status;
        (*_ti).syscall_times = COUNT_SYSCALL_TIMES.exclusive_access().clone();
        (*_ti).time = get_time();
    }
    0
}
