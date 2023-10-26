//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{get_curr_task_id,fetch_curr_task_control_block, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::{get_time_us, get_time},
    sync::UPSafeCell
};
use lazy_static::*;

type SyscallCountInfo = [u32; MAX_SYSCALL_NUM];

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
    syscall_times: SyscallCountInfo,
    /// Total running time of task
    time: usize,
}

lazy_static! {
    pub static ref TASK_SYSCALL_TIMES: 
    UPSafeCell<HashMap::<usize, SyscallCountInfo>> = 
        unsafe{
            UPSafeCell::new(HashMap::<usize, SyscallCountInfo>::new())
        };
}

/// 增加当前task的目标syscall计数
pub fn increase_curr_task_syscall_times(syscall_id: &u32) {
    TASK_SYSCALL_TIMES.exclusive_access()
        .entry(get_curr_task_id())
        .and_modify(|counter| (*counter)[syscall_id] += 1)
        .or_insert(1);
}

/// 获取目标task的syscall计数列表
pub fn get_task_syscall_times(task_id: &usize) -> u32 {
    TASK_SYSCALL_TIMES.exclusive_access()
    .get(task_id).unwrap_or([0; MAX_SYSCALL_NUM])
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
        (*_ti).syscall_times = get_task_syscall_times(get_curr_task_id());
        (*_ti).time = get_time();
    }
    0
}
