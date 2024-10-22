use crate::task::{Task, TaskState};
use chrono::{DateTime, Utc};

pub fn display_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        display_task(task);
    }
}
pub fn display_task(task: &Task) {
    let task_status = match task.state {
        TaskState::Pending => " ",
        TaskState::Completed => "x",
        TaskState::OnHold => "-",
        TaskState::NeedInfo => "?",
        TaskState::InProgress => "…",
        TaskState::Cancelled => "!",
    };

    println!("\n\t[{}]\t{}", task_status, task.title);
    if task.description.is_some() {
        println!("\t\t{}", task.description.clone().unwrap())
    };
    println!(
        "\t\tPriority:{}\t\tDue date: {}",
        task.priority.clone().unwrap(),
        (if task.due_date.is_some() {
            DateTime::<Utc>::from(task.due_date.unwrap())
                .format("%Y/%m/%d")
                .to_string()
        } else {
            "None".to_string()
        })
    );
}
