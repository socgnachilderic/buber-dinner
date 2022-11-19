use uuid::Uuid;

use crate::common::models::ValueObject;

pub use dinner_id::*;

mod dinner_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct DinnerId(super::Uuid);

    impl<'a> super::ValueObject<'a> for DinnerId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for DinnerId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}
