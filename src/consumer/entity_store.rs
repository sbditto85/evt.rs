use crate::consumer::entity_cache::EntityCache;
use crate::message_store::{Get, MessageData, MessageStore};
use crate::{messaging::Message, stream_name, Error};
use serde::{de::DeserializeOwned, Serialize};

use std::convert::TryFrom;

pub trait EntityStoreEntity: Serialize + DeserializeOwned + Default + Clone {
    type Projector: EntityBuilder<Self>;
    fn get_projector() -> Self::Projector;
}

pub trait EntityStore<T: EntityStoreEntity> {
    type Cache: EntityCache<T>;
    fn get_category(&self) -> String;
    fn get_store(&mut self) -> &mut MessageStore;
    fn get_cache(&mut self) -> &mut Self::Cache;

    fn fetch(&mut self, identity: &str) -> Result<T, Error> {
        let category = &self.get_category();
        let entity_info: Option<(i64, T)> = self.get_cache().get_from_cache(category, identity);
        let mut position = -1;
        let mut entity_builder = T::get_projector();
        if let Some((cached_position, cached_entity)) = entity_info {
            position = cached_position;
            entity_builder.initialize(cached_entity);
        }
        loop {
            let messages = self
                .get_store()
                .get(&stream_name!(category, id = identity), Some(position))?;
            let messages_length = messages.len();

            // handle all the messages
            for message_data in messages {
                // Position should be set as were reading
                if let Some(message_position) = message_data.position {
                    position = message_position;
                }
                entity_builder.apply(message_data);
            }

            // TODO: make 1000 a const somewhere and make sure it matches messagedbs default
            if messages_length < self.get_store().settings.batch_size.unwrap_or(1000) as usize {
                break;
            }
        }

        let entity = entity_builder.entity();

        self.get_cache()
            .set_in_cache(category, identity, position, entity.clone());
        Ok(entity)
    }
    // fn get<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<Option<T>, Error>;
}

pub trait Projection<T: TryFrom<MessageData> + Default + Serialize + DeserializeOwned> {
    fn apply(&mut self, message: &Message<T>);
}

pub trait EntityBuilder<T> {
    fn initialize(&mut self, base_entity: T);
    fn apply(&mut self, message_data: MessageData);
    fn entity(&mut self) -> T;
}

// Made this up, verify with eventide later
pub trait EntitySnapShot {
    fn snapshot(&mut self, category: &str, identity: &str, position: u64, entity: Self);
    fn retrieve(&mut self, category: &str, identity: &str) -> (u64, Self);
}
