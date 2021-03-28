use crate::consumer::Handler;
use crate::message_store::{Get, MessageData, MessageStore, Put};

#[derive(Default)]
pub struct Settings {
    // snapshot settings etc
    pub batch_size: Option<i64>,                   // batch_size
    pub correlation: Option<String>,               // correlation
    pub group_member: Option<i64>,                 // group_member
    pub group_size: Option<i64>,                   // group_size
    pub condition: Option<String>,                 // condition
    pub poll_interfaval_milliseconds: Option<u64>, // poll_interval_milliseconds
    pub position_update_interval: Option<i64>,     // position_update_interval
    pub identifier: Option<String>,                // identifier
}
