//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, get_current_mem_set,
    }, 
    mm::{translated_byte_buffer, VirtAddr, MapPermission, VirtPageNum}, timer::get_time_us,
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
    trace!("kernel: sys_mmap");
    println!("DEBUG: kernel: sys_mmap 0x{:08x}, {}, {:08o}", _start, _len, _port);

    let va_start = VirtAddr::from(_start);
    let va_end = VirtAddr::from(_start + _len); //右侧是开区间

    if _len == 0 {
        return 0;
    }
    if !va_start.aligned(){
        //println!("DEBUG: mmap: not aligned");
        return -1;
    }
    if _port & !0x7 != 0 || _port * 0x7 == 0 {
        return -1;
    }

    let start_vpn: VirtPageNum = va_start.floor();
    let end_vpn: VirtPageNum = va_end.ceil();
    println!("DEBUG: mmap vpn [{:?}, {:?})", start_vpn, end_vpn);
    
    let mem_set = get_current_mem_set();

    //有被映射过的页
    for vpn in usize::from(start_vpn) ..usize::from(end_vpn) {
        if let Some(_) = mem_set.translate(VirtPageNum::from(vpn)) {
            return -1;
        }
    }

    //物理内存不足 TODO 真需要考虑吗？

    // let mut map_perm = MapPermission::U;
    // if ph_flags.is_read() {
    //     map_perm |= MapPermission::R;
    // }
    // if ph_flags.is_write() {
    //     map_perm |= MapPermission::W;
    // }
    // if ph_flags.is_execute() {
    //     map_perm |= MapPermission::X;
    // }
    mem_set.insert_framed_area(va_start, va_end,
        MapPermission::from_bits_truncate(_port as u8 | (1<<3)));
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    println!("DEBUG: kernel: sys_munmap 0x{:08x}, {}", _start, _len);

    let va_start = VirtAddr::from(_start);
    let va_end = VirtAddr::from(_start + _len - 1);

    if _len == 0 {
        return 0;
    }
    if !va_start.aligned(){
        return -1;
    }

    let start_vpn: VirtPageNum = va_start.floor();
    let end_vpn: VirtPageNum = va_end.ceil();
    
    let mem_set = get_current_mem_set();

    //有没被映射过的页
    for vpn in usize::from(start_vpn) ..usize::from(end_vpn) {
        if let None = mem_set.translate(VirtPageNum::from(vpn)) {
            return -1;
        }
    }

    for vpn in usize::from(start_vpn) ..usize::from(end_vpn) {
        mem_set.unmap_vpn(VirtPageNum::from(vpn));
    }

    0
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
