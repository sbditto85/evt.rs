use log::warn;
use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use crate::consumer::position_store::PositionStore;
use crate::consumer::Settings;
use crate::message_store::{MessageData, MessageStore};
use crate::messaging::{Message, MessageType};
use crate::Error;

const POLL_INTERVAL_MILLISECONDS_DEFAULT: u64 = 1000; // TODO: make sure this works
const STARTING_POSITION: i64 = 0; // TODO: verify its 0 or -1 for subscription

pub struct Consumer<B: BackOff> {
    category: String,
    settings: Settings,
    store: MessageStore,
    current_position: i64,
    back_off: B,
    handlers: HashMap<String, fn(message_data: MessageData) -> Result<(), Error>>,
    should_continue: Arc<Mutex<bool>>,
}

impl Consumer<SimpleBackOff> {
    pub fn new(category: String, store: MessageStore, settings: Settings) -> Self {
        let poll_interval_milliseconds = settings
            .poll_interval_milliseconds
            .unwrap_or(POLL_INTERVAL_MILLISECONDS_DEFAULT);
        Self {
            category,
            settings,
            store,
            current_position: STARTING_POSITION,
            back_off: SimpleBackOff {
                poll_interval_milliseconds,
            },
            handlers: HashMap::new(),
            should_continue: Arc::new(Mutex::new(true)),
        }
    }

    // pub fn add_handler<T>(&mut self, handler: fn(message_data: MessageData) -> Result<(), Error>)
    // where
    //     T: MessageType + Default + Serialize + DeserializeOwned,
    //     Message<T>: TryFrom<MessageData>,
    // {
    //     self.handlers
    //         .entry(T::message_type())
    //         .and_modify(|current_handler| {
    //             warn!("Re-assinging handler for {}", T::message_type());
    //             *current_handler = handler;
    //         })
    //         .or_insert(handler);
    // }

    pub fn start(&mut self) -> Result<(), Error> {
        self.poll_continuously()
    }

    pub fn stopper(&self) -> impl Stopper {
        ConsumerStopper {
            should_continue: self.should_continue.clone(),
        }
    }

    fn poll_continuously(&mut self) -> Result<(), Error> {
        self.load_position()?;

        let lock = self
            .should_continue
            .lock()
            .map_err(|_| Error::ConsumerError)?;
        let mut should_continue = *lock;

        // So that poll can be called
        drop(lock);

        while should_continue {
            let messages_processed = self.poll()?;

            self.back_off.wait(messages_processed);

            // lock and check again
            let lock = self
                .should_continue
                .lock()
                .map_err(|_| Error::ConsumerError)?;
            should_continue = *lock;
        }

        Ok(())
    }

    fn load_position(&mut self) -> Result<(), Error> {
        let last_position = self.get_last(self.settings.identifier.clone().as_deref())?;
        self.current_position = last_position.unwrap_or(STARTING_POSITION);
        Ok(())
    }

    fn poll(&mut self) -> Result<u64, Error> {
        Ok(todo!())
    }
}

impl<B: BackOff> PositionStore for Consumer<B> {
    fn get_category(&self) -> String {
        self.category.clone()
    }
    fn get_store(&mut self) -> &mut MessageStore {
        &mut self.store
    }
}

pub trait BackOff {
    fn wait(&mut self, messages_processed: u64);
}

struct SimpleBackOff {
    poll_interval_milliseconds: u64,
}

impl BackOff for SimpleBackOff {
    fn wait(&mut self, messages_processed: u64) {
        if messages_processed == 0 {
            thread::sleep(Duration::from_millis(self.poll_interval_milliseconds));
        }
    }
}

pub trait Stopper {
    fn stop(&mut self) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct ConsumerStopper {
    should_continue: Arc<Mutex<bool>>,
}

impl Stopper for ConsumerStopper {
    fn stop(&mut self) -> Result<(), Error> {
        let mut lock = self.should_continue.lock().map_err(|mut poisoned_error| {
            let should_continue = poisoned_error.get_mut();

            **should_continue = false;

            Error::ConsumerError
        })?;

        *lock = false;

        Ok(())
    }
}

pub trait Handler {
    fn handle<T: TryFrom<MessageData>>(&mut self, message: T) -> Result<(), Error>;
}
