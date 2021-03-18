use crate::consumer::Settings;
use crate::message_store::MessageData;
use crate::Error;

use std::convert::TryFrom;

pub trait Consumer {
    type Stop: Stop;
    fn start(&mut self, settings: Settings) -> self::Stop;
}

pub trait Stop {
    fn stop(&mut self);
}

pub trait Handler {
    fn handle<T: TryFrom<MessageData>>(&mut self, message: T) -> Result<(), Error>;
}
