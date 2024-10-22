#![allow(unused, dead_code)]
use crate::user::{Contact, User};

use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}
impl Tasks {
    pub fn new() -> Self {
        Tasks { tasks: vec![] }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "system_time_format", default)]
    pub due_date: Option<SystemTime>,
    #[serde(default)]
    pub state: TaskState,
    pub priority: Option<TaskPriority>,
    pub time_estimation_minutes: Option<u32>,
    pub tags: Option<HashSet<String>>,
    #[serde(with = "system_time_format", default = "default_system_time")]
    pub created_at: Option<SystemTime>,
    #[serde(with = "system_time_format", default = "default_system_time")]
    pub modified_at: Option<SystemTime>,
    #[serde(with = "system_time_format", default)]
    pub deleted_at: Option<SystemTime>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum TaskState {
    #[default]
    Pending,
    NeedInfo,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}
impl Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Critical => write!(f, "Critical"),
        }
    }
}

pub fn default_system_time() -> Option<SystemTime> {
    Some(SystemTime::now())
}
mod system_time_format {
    use chrono::{NaiveDate, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(system_time) => {
                let datetime = chrono::DateTime::<Utc>::from(*system_time);
                let formatted = datetime.format(FORMAT).to_string();
                serializer.serialize_str(&formatted)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        if let Some(date_str) = opt {
            let naive_date =
                NaiveDate::parse_from_str(&date_str, FORMAT).map_err(serde::de::Error::custom)?;
            let datetime = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
            let system_time = SystemTime::from(datetime);
            Ok(Some(system_time))
        } else {
            Ok(None)
        }
    }
}
