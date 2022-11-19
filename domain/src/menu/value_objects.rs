use uuid::Uuid;

use crate::common::models::ValueObject;

pub use menu_id::*;
pub use menu_item_id::*;
pub use menu_section_id::*;

mod menu_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct MenuId(super::Uuid);

    impl<'a> super::ValueObject<'a> for MenuId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for MenuId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}

mod menu_section_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct MenuSectionId(super::Uuid);

    impl<'a> super::ValueObject<'a> for MenuSectionId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for MenuSectionId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}

mod menu_item_id {
    #[derive(Debug, Clone, PartialEq)]
    pub struct MenuItemId(super::Uuid);

    impl<'a> super::ValueObject<'a> for MenuItemId {
        type Value = super::Uuid;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }

    impl Default for MenuItemId {
        fn default() -> Self {
            Self(super::Uuid::new_v4())
        }
    }
}
