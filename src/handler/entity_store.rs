use serde::{de::DeserializeOwned, Serialize};
use std::ops::Deref;

use crate::handler::Handler;
use crate::message_store::{Get, MessageData, MessageStore, Put};
use crate::stream_name;
use crate::Error;

pub trait EntityStoreEntity:
    EntitySnapShot + Serialize + DeserializeOwned + Default + Clone
{
    type Projector: EntityBuilder<Self>;
    fn get_projector() -> Self::Projector;
}

pub trait EntityStore {
    fn fetch<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<T, Error>;
    fn get<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<Option<T>, Error>;
}

impl EntityStore for Handler {
    fn fetch<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<T, Error> {
        let category = &self.category.clone();
        let entity_info: Option<(i64, T)> = self.get_from_cache(category, identity);
        let mut position = -1;
        let mut entity_builder = T::get_projector();
        if let Some((cached_position, cached_entity)) = entity_info {
            position = cached_position;
            entity_builder.initialize(cached_entity);
        }
        loop {
            let messages = (&mut self.store as &mut dyn Get)
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
            if messages_length < self.store.settings.batch_size.unwrap_or(1000) as usize {
                break;
            }
        }

        let entity = entity_builder.entity();

        self.set_in_cache(category, identity, position, entity.clone());
        Ok(entity)
    }

    fn get<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<Option<T>, Error> {
        let mut has_entity = false;

        let category = &self.category.clone();
        let entity_info: Option<(i64, T)> = self.get_from_cache(category, identity);
        let mut position = -1;
        let mut entity_builder = T::get_projector();
        if let Some((cached_position, cached_entity)) = entity_info {
            position = cached_position;
            entity_builder.initialize(cached_entity);
        }
        loop {
            let messages = (&mut self.store as &mut dyn Get)
                .get(&stream_name!(category, id = identity), Some(position))?;
            let messages_length = messages.len();

            if messages_length > 0 {
                has_entity = true;
            }

            // handle all the messages
            for message_data in messages {
                // Position should be set as were reading
                if let Some(message_position) = message_data.position {
                    position = message_position;
                }
                entity_builder.apply(message_data);
            }

            // TODO: make 1000 a const somewhere and make sure it matches messagedbs default
            if messages_length < self.store.settings.batch_size.unwrap_or(1000) as usize {
                break;
            }
        }

        if has_entity {
            let entity = entity_builder.entity();
            self.set_in_cache(category, identity, position, entity.clone());
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }
}

pub trait Projection<E> {
    fn apply(&self, entity: &mut E);
}

pub trait EntityBuilder<T> {
    fn initialize(&mut self, base_entity: T);
    fn apply(&mut self, message: MessageData);
    fn entity(&mut self) -> T;
}

// Made this up, verify with eventide later
pub trait EntitySnapShot {
    fn snapshot(&mut self, identity: &str, position: u64, entity: Self);
    fn retrieve(&mut self, identity: &str) -> (u64, Self);
}
