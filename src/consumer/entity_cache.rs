use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;

pub trait EntityCache<T: Serialize + DeserializeOwned> {
    fn get_from_cache(&self, category: &str, identity: &str) -> Option<(i64, T)>;
    fn set_in_cache(&mut self, category: &str, identity: &str, position: i64, value: T);
}

pub struct DontCache;
impl<T: Serialize + DeserializeOwned> EntityCache<T> for DontCache {
    fn get_from_cache(&self, _category: &str, _identity: &str) -> Option<(i64, T)> {
        None
    }
    fn set_in_cache(&mut self, _category: &str, _identity: &str, _position: i64, _value: T) {}
}

pub struct InMemoryCache {
    entities: HashMap<String, HashMap<String, (i64, serde_json::Value)>>,
}
impl<T: Serialize + DeserializeOwned> EntityCache<T> for InMemoryCache {
    fn get_from_cache(&self, category: &str, identity: &str) -> Option<(i64, T)>
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

    fn set_in_cache(&mut self, category: &str, identity: &str, position: i64, value: T)
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
