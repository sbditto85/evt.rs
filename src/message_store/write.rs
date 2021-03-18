use crate::message_store::{MessageData, MessageStore, Put, INITIAL};
use crate::Error;

pub trait WriteMessage {
    fn write<M: Into<MessageData>>(&mut self, message: M, stream_name: &str) -> Result<(), Error>;

    fn write_position<M: Into<MessageData>>(
        &mut self,
        message: M,
        stream_name: &str,
        expected_version: i64,
    ) -> Result<(), Error>;

    // TODO: write_many?

    fn initial<M: Into<MessageData>>(&mut self, message: M, stream_name: &str)
        -> Result<(), Error>;
}

impl WriteMessage for MessageStore {
    fn write<M: Into<MessageData>>(&mut self, message: M, stream_name: &str) -> Result<(), Error> {
        self.put(&message.into(), stream_name, None)?;
        Ok(())
    }
    fn write_position<M: Into<MessageData>>(
        &mut self,
        message: M,
        stream_name: &str,
        expected_version: i64,
    ) -> Result<(), Error> {
        self.put(&message.into(), stream_name, Some(expected_version))?;
        Ok(())
    }
    fn initial<M: Into<MessageData>>(
        &mut self,
        message: M,
        stream_name: &str,
    ) -> Result<(), Error> {
        self.put(&message.into(), stream_name, INITIAL)?;
        Ok(())
    }
}