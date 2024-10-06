#![allow(unused, dead_code, unused_imports)]
use crate::task::Task;
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    title: String,
    birth_date: DateTime<Utc>,
    birth_place: String,
    age: i32,
    email: String,
    username: String,
    profile_picture_url: Url,
    preferences: Preferences,
    completed_tasks: Vec<Task>,
    current_level: i32,
    xp_points: i32,
}

pub struct Contact {}

struct Url {
    url: String,
}

struct Preferences {
    theme: String,
    notification_settings: NotificationSettings,
    language: String,
    time_format: String,
}

struct NotificationSettings {
    email_notifications: bool,
    push_notifications: bool,
    reminder_time: Option<DateTime<Utc>>,
}
