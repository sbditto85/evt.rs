use crate::stream_name;
use crate::stream_name::utils::{get_category_types, is_category};
use crate::Error;

const POSITION_TYPE: &'static str = "position";

pub trait PositionStore {
    fn get_last(&mut self) -> Result<Option<i64>, Error>;
    fn update(&mut self, position: i64) -> Result<(), Error>;
    fn position_stream_name(
        stream_name: &str,
        consumer_identifier: Option<&str>,
    ) -> Option<String> {
        if is_category(stream_name) {
            let postition_type = POSITION_TYPE.to_string();
            let category_types = if let Some(mut types) = get_category_types(stream_name) {
                if types.contains(&postition_type) {
                    types
                } else {
                    types.push(postition_type);
                    types
                }
            } else {
                vec![postition_type]
            };

            let position_stream_name = if let Some(consumer) = consumer_identifier {
                stream_name!(stream_name, category_types = category_types, id = consumer)
            } else {
                stream_name!(stream_name, category_types = category_types)
            };
            Some(position_stream_name)
        } else {
            None
        }
    }
}
