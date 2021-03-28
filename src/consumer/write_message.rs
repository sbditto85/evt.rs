use serde::de::DeserializeOwned;
use serde::Serialize;

// use crate::message_store::MessageData;
use crate::messaging::{write::Write, Message};
use crate::{Error, MessageStore};

pub trait WriteMessage {
    fn get_store(&mut self) -> &mut MessageStore;

    fn write<T>(
        &mut self,
        batch: &Message<T>,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned + Default,
    {
        self.get_store().write(batch, stream_name, expected_version)
    }

    fn write_initial<T>(&mut self, batch: &Message<T>, stream_name: &str) -> Result<(), Error>
    where
        T: Serialize + DeserializeOwned + Default,
    {
        self.get_store().write_initial(batch, stream_name)
    }
}
