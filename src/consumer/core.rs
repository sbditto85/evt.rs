use crate::consumer::Handler;
use crate::message_store::{Get, MessageData, MessageStore, Put, WriteMessage};

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

pub struct BasicConsumer<R: Get, W: WriteMessage, H: Handler> {
    pub(crate) settings: Settings,
    pub(crate) reader: R,
    pub(crate) writer: W,
    pub(crate) handler: H,
}

impl<R: Get, W: WriteMessage, H: Handler> BasicConsumer<R, W, H> {
    pub fn build(reader: R, writer: W, handler: H) -> Self {
        Self {
            settings: Settings::default(),
            reader,
            writer,
            handler,
        }
    }

    pub fn build_with_settings(settings: Settings, reader: R, writer: W, handler: H) -> Self {
        Self {
            settings,
            reader,
            writer,
            handler,
        }
    }
}
