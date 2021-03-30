use serde::de::DeserializeOwned;
use serde::Serialize;

// use crate::message_store::MessageData;
use crate::messaging::{write::Write, Message};
use crate::{stream_name, Error, MessageStore};

pub trait WriteMessage {
    fn get_category(&self) -> String;
    fn get_store(&mut self) -> &mut MessageStore;

    fn write<T>(
        &mut self,
        message: &Message<T>,
        identity: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned + Default,
    {
        let category = self.get_category();
        self.get_store().write(
            message,
            &stream_name!(category, id = identity),
            expected_version,
        )
    }

    fn write_initial<T>(&mut self, message: &Message<T>, identity: &str) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned + Default,
    {
        let category = self.get_category();
        self.get_store()
            .write_initial(message, &stream_name!(category, id = identity))
    }
}
