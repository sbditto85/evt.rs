use crate::consumer::Settings;
use crate::message_store::MessageData;
use crate::Error;

use std::convert::TryFrom;

pub trait Consumer {
    type Stop: Stop;
    fn get_category() -> String;
    fn start(&mut self, settings: Settings) -> dyn self::Stop;
}

// pub struct BasicConsumer {
//     store: MessageStore,
//     handler: Box<&mut dyn Handler>,
// }
// impl Consumer for BasicConsumer {
//     type Stop = ();
//     fn start(&mut self, settings: Settings) -> self::Stop {
//         // setup the polling etc inject the handler

//         loop {
//             // have message
//             if let Err(proble) = messages.map(|message| handle(message)).collect() {
//                 AHHHHHH
//             }
//         }
//         ()
//     }
// }

pub trait Stop {
    fn stop(&mut self);
}

pub trait Handler {
    fn handle<T: TryFrom<MessageData>>(&mut self, message: T) -> Result<(), Error>;
}
