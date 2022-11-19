use crate::common::models::Entity;

pub use menu_item::*;
pub use menu_section::*;

mod menu_section {
    use super::MenuItem;
    use crate::menu::value_objects::MenuSectionId;

    #[derive(Debug)]
    pub struct MenuSection {
        id: MenuSectionId,
        menu_items: Vec<MenuItem>,
        pub name: String,
        pub description: String,
    }

    impl MenuSection {
        pub fn new(name: &str, description: &str) -> Self {
            Self {
                id: MenuSectionId::default(),
                menu_items: vec![],
                name: name.to_string(),
                description: description.to_string(),
            }
        }

        pub fn get_items<'a>(&'a self) -> &'a [MenuItem] {
            &self.menu_items
        }
    }

    impl super::Entity for MenuSection {
        type Id = MenuSectionId;

        fn get_id<'a>(&'a self) -> &'a Self::Id {
            todo!()
        }
    }

    impl PartialEq for MenuSection {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
}

mod menu_item {
    use crate::menu::value_objects::MenuItemId;

    #[derive(Debug)]
    pub struct MenuItem {
        id: MenuItemId,
        pub name: String,
        pub description: String,
    }

    impl MenuItem {
        pub fn new(name: &str, description: &str) -> Self {
            Self {
                id: MenuItemId::default(),
                name: name.to_string(),
                description: description.to_string(),
            }
        }
    }

    impl super::Entity for MenuItem {
        type Id = MenuItemId;

        fn get_id<'a>(&'a self) -> &'a Self::Id {
            &self.id
        }
    }

    impl PartialEq for MenuItem {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
}
