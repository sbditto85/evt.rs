use crate::handler::Handler;
use crate::message_store::MessageData;
use crate::Error;

use std::convert::TryFrom;

pub struct Settings {}

pub trait Consumer {
    fn start(&mut self, settings: Settings);
    fn handle<T: TryFrom<MessageData>>(&mut self, message: T) -> Result<(), Error>;
}

impl Consumer for Handler {
    fn start(&mut self, settings: Settings) {
        todo!()
    }

    fn handle<T: TryFrom<MessageData>>(&mut self, message: T) -> Result<(), Error> {
        todo!()
    }
}
