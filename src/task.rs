#![allow(unused, dead_code)]
use crate::user::{Contact, User};

use chrono::Duration;
use std::time::SystemTime;
use uuid::Uuid;

pub struct Task {
    id: Uuid,
    title: String,
    input_complete: bool,
    description: Option<String>,
    creation_date: SystemTime,
    due_date: Option<SystemTime>,
    modified_date: Option<SystemTime>,
    creator: User,
    assigner: Contact,
    progress: TaskProgress,
    done_date: Option<SystemTime>,
    sub_tasks: Option<Vec<Task>>,
    priority: Option<TaskPriority>,
    time_estimation: Duration,
    time_spent: Duration,
    tags: Option<Vec<String>>,
}

enum TaskProgress {
    Pending,
    NeedInfo,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}
