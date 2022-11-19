use uuid::Uuid;

use crate::common::models::ValueObject;

pub use menu_review_id::*;

mod menu_review_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct MenuReviewId(super::Uuid);

    impl<'a> super::ValueObject<'a> for MenuReviewId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for MenuReviewId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}
