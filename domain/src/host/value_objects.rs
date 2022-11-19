use uuid::Uuid;

use crate::common::models::ValueObject;

pub use host_id::*;

mod host_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct HostId(super::Uuid);

    impl<'a> super::ValueObject<'a> for HostId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for HostId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}
