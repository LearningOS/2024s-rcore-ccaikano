//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use crate::timer::get_time_ms;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info
    pub task_info: TaskInfoBlock,
}

#[derive(Clone, Copy)]
pub struct TaskInfoBlock {
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// 开始运行时间
    pub start_time: usize,
}

impl TaskInfoBlock {
    /// Create a new task info
    pub fn new() -> Self {
        TaskInfoBlock {
            syscall_times: [0; MAX_SYSCALL_NUM],
            start_time: get_time_ms(),
        }
    }
    /// Increment the syscall times
    pub fn increment_syscall_times(&mut self, syscall_id: usize, times: usize) {
        self.syscall_times[syscall_id] += times as u32;
    }
}
/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
