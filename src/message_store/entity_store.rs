use crate::message_store::{Get, MessageData, MessageStore, Put};
use crate::stream_name;
use crate::Error;
use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;

pub trait EntityStoreEntity:
    EntityCategory + EntitySnapShot + Serialize + DeserializeOwned + Default + Clone
{
    type Projector: EntityBuilder<Self>;
    fn get_projector() -> Self::Projector;
}

pub trait EntityStore {
    fn fetch<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<T, Error>;
    fn get<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<Option<T>, Error>;
}

impl EntityStore for MessageStore {
    fn fetch<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<T, Error> {
        let category = T::get_category();
        let entity_info: Option<(i64, T)> = self.get_from_cache(category, identity);
        let mut position = -1;
        let mut entity_builder = T::get_projector();
        if let Some((cached_position, cached_entity)) = entity_info {
            position = cached_position;
            entity_builder.initialize(cached_entity);
        }
        loop {
            let messages = (self as &mut dyn Get)
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
            if messages_length < self.settings.batch_size.unwrap_or(1000) as usize {
                break;
            }
        }

        let entity = entity_builder.entity();

        self.set_in_cache(category, identity, position, entity.clone());
        Ok(entity)
    }

    fn get<T: EntityStoreEntity>(&mut self, identity: &str) -> Result<Option<T>, Error> {
        let mut has_entity = false;

        let category = T::get_category();
        let entity_info: Option<(i64, T)> = self.get_from_cache(category, identity);
        let mut position = -1;
        let mut entity_builder = T::get_projector();
        if let Some((cached_position, cached_entity)) = entity_info {
            position = cached_position;
            entity_builder.initialize(cached_entity);
        }
        loop {
            let messages = (self as &mut dyn Get)
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
            if messages_length < self.settings.batch_size.unwrap_or(1000) as usize {
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

pub trait EntityCategory {
    fn get_category() -> &'static str;
}

pub trait EntityCache {
    fn get_from_cache<T>(&self, category: &str, identity: &str) -> Option<(i64, T)>
    where
        T: DeserializeOwned;

    fn set_in_cache<T>(&mut self, category: &str, identity: &str, position: i64, value: T)
    where
        T: Serialize;
}

impl EntityCache for MessageStore {
    fn get_from_cache<T>(&self, category: &str, identity: &str) -> Option<(i64, T)>
    where
        T: DeserializeOwned,
    {
        self.entities
            .get(category)
            .and_then(|entities| entities.get(&identity.to_string()))
            .and_then(|(position, entity_value)| {
                serde_json::from_value(entity_value.clone())
                    .ok()
                    .map(|entity| (*position, entity))
            })
    }

    fn set_in_cache<T>(&mut self, category: &str, identity: &str, position: i64, value: T)
    where
        T: Serialize,
    {
        let json_value = serde_json::to_value(value).expect("entity to serialize");
        self.entities
            .entry(category.to_string())
            .and_modify(|entities| {
                entities
                    .entry(identity.to_string())
                    .and_modify(|entry_info| *entry_info = (position, json_value.clone()))
                    .or_insert((position, json_value.clone()));
            })
            .or_insert(
                vec![(identity.to_string(), (position, json_value))]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            );
    }
}

// Made this up, verify with eventide later
pub trait EntitySnapShot {
    fn snapshot(&mut self, identity: &str, position: u64, entity: Self);
    fn retrieve(&mut self, identity: &str) -> (u64, Self);
}
