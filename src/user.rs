#![allow(unused, dead_code, unused_imports)]
use crate::task::Task;
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

// TODO: Add options
#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    contact: Contact,
    birth_date: Option<DateTime<Utc>>,
    birth_place: Option<String>,
    age: Option<i32>,
    username: Option<String>,
    profile_picture_url: Option<Url>,
    preferences: Preferences,
    completed_tasks: Option<Vec<Task>>,
    current_level: i32,
    xp_points: i32,
}

#[derive(Debug, Clone)]
pub struct Contact {
    name: String,
    title: String,
    email: String,
}

#[derive(Debug, Clone)]
struct Url {
    url: String,
}

#[derive(Debug, Clone)]
struct Preferences {
    theme: String,
    notification_settings: NotificationSettings,
    language: String,
    time_format: String,
}

#[derive(Debug, Clone)]
struct NotificationSettings {
    email_notifications: bool,
    push_notifications: bool,
    reminder_time: Option<DateTime<Utc>>,
}
