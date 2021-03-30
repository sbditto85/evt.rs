use serde::{Deserialize, Serialize};

use crate::message_store::{Get, MessageStore};
use crate::messaging::{Message, MessageType, Write};
use crate::stream_name;
use crate::stream_name::utils::{get_category_types, is_category};
use crate::Error;

use std::convert::TryFrom;

const POSITION_TYPE: &'static str = "position";

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
struct Position {
    position: i64,
}

impl MessageType for Position {
    fn message_type() -> String {
        String::from("Position")
    }
}

pub trait PositionStore {
    fn get_category(&self) -> String;
    fn get_store(&mut self) -> &mut MessageStore;

    fn get_last(&mut self, consumer_identity: Option<&str>) -> Result<Option<i64>, Error> {
        let position_stream_name =
            Self::position_stream_name(&self.get_category(), consumer_identity)
                .ok_or(Error::StreamName)?;

        let last_message_data = self.get_store().get_last(&position_stream_name)?;

        Ok(last_message_data.and_then(|message_data| {
            Message::try_from(message_data)
                .ok()
                .map(|position: Message<Position>| position.into_inner().position)
        }))
    }

    fn update(&mut self, consumer_identity: Option<&str>, position: i64) -> Result<(), Error> {
        let position_stream_name =
            Self::position_stream_name(&self.get_category(), consumer_identity)
                .ok_or(Error::StreamName)?;

        let position = Position { position };
        let message = Message::from_t(position);

        self.get_store()
            .write(&message, &position_stream_name, None)
    }

    fn position_stream_name(
        stream_name: &str,
        consumer_identifier: Option<&str>,
    ) -> Option<String> {
        if is_category(stream_name) {
            let postition_type = POSITION_TYPE.to_string();
            let category_types = if let Some(mut types) = get_category_types(stream_name) {
                if types.contains(&postition_type) {
                    types
                } else {
                    types.push(postition_type);
                    types
                }
            } else {
                vec![postition_type]
            };

            let position_stream_name = if let Some(consumer) = consumer_identifier {
                stream_name!(stream_name, category_types = category_types, id = consumer)
            } else {
                stream_name!(stream_name, category_types = category_types)
            };
            Some(position_stream_name)
        } else {
            None
        }
    }
}
