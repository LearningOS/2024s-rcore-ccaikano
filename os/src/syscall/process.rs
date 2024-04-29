//! Process management syscalls
use alloc::boxed::Box;
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};
use crate::task::TASK_MANAGER;
use crate::timer::get_time_ms;

/// The time
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// The number of seconds
    pub sec: usize,
    /// The number of microseconds
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Clone, Copy)]
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
    let task_manager = TASK_MANAGER.inner.exclusive_access();
    let current_task_index = task_manager.current_task;

    // 提前取出需要的值
    let syscall_times = task_manager.tasks[current_task_index].task_info.syscall_times.clone();
    let status = task_manager.tasks[current_task_index].task_status;
    let start_time = task_manager.tasks[current_task_index].task_info.start_time;


    let task_info =  Box::new(TaskInfo {
        status,
        syscall_times, // 使用预先复制的值
        time: get_time_ms() - start_time, // 使用预先复制的值
    });
    unsafe {
        *_ti = *task_info;
    }
    drop(task_manager);
    0
}
