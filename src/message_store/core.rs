use crate::db;
use crate::Json;
use crate::Uuid;
use crate::{DateTime, Utc};
use postgres::Client;

use std::collections::HashMap;

pub const INITIAL: Option<i64> = Some(-1);

#[derive(Default, Clone)]
pub struct MessageData {
    pub id: Option<Uuid>,
    pub message_type: String,
    pub stream_name: Option<String>,
    pub position: Option<i64>,
    pub global_position: Option<i64>,
    pub data: Json,
    pub metadata: Json,
    pub time: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct Settings {
    pub batch_size: Option<i64>,
    pub correlation: Option<String>,
    pub group_member: Option<i64>,
    pub group_size: Option<i64>,
    pub condition: Option<String>,
}

pub struct MessageStore {
    pub settings: Settings,
    pub client: Client,
    pub(crate) entities: HashMap<String, HashMap<String, (i64, serde_json::Value)>>,
}

impl MessageStore {
    pub fn build() -> Self {
        MessageStore {
            settings: Settings::default(),
            client: db::build(),
            entities: HashMap::new(),
        }
    }

    pub fn build_with_settings(settings: Settings) -> Self {
        MessageStore {
            settings,
            client: db::build(),
            entities: HashMap::new(),
        }
    }
}
