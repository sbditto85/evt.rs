use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

use crate::message_store::MessageStore;

#[derive(Default)]
pub struct Settings {
    // snapshot settings etc
}

pub struct Handler {
    pub(crate) category: String,
    pub(crate) store: MessageStore,
    pub(crate) settings: Settings,
    entities: HashMap<String, HashMap<String, (i64, serde_json::Value)>>,
}

impl Handler {
    pub fn build(category: String) -> Self {
        Self {
            category,
            store: MessageStore::build(),
            settings: Settings::default(),
            entities: HashMap::new(),
        }
    }

    pub(crate) fn get_from_cache<T>(&self, category: &str, identity: &str) -> Option<(i64, T)>
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

    pub(crate) fn set_in_cache<T>(
        &mut self,
        category: &str,
        identity: &str,
        position: i64,
        value: T,
    ) where
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
