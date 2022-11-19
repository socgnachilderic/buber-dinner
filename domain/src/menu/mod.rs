use chrono::{DateTime, Utc};

use crate::common::models::{AggregateRoot, Entity};
use crate::common::value_objects::AverageRating;
use crate::dinner::value_objects::DinnerId;
use crate::host::value_objects::HostId;
use crate::menu_review::value_objects::MenuReviewId;

use entities::MenuSection;
use value_objects::MenuId;

pub mod entities;
pub mod value_objects;

#[derive(Debug)]
pub struct Menu {
    id: MenuId,
    sections: Vec<MenuSection>,
    dinner_ids: Vec<DinnerId>,
    menu_review_ids: Vec<MenuReviewId>,
    pub average_rating: AverageRating,
    pub name: String,
    pub description: String,
    pub host_id: HostId,
    pub created_date_time: DateTime<Utc>,
    pub updated_date_time: DateTime<Utc>,
}

impl<'a> Menu {
    pub fn new(name: &str, description: &str, host_id: &HostId) -> Self {
        Self {
            id: MenuId::default(),
            sections: vec![],
            dinner_ids: vec![],
            menu_review_ids: vec![],
            name: name.to_string(),
            description: description.to_string(),
            average_rating: AverageRating::default(),
            host_id: host_id.to_owned(),
            created_date_time: Utc::now(),
            updated_date_time: Utc::now(),
        }
    }

    pub fn get_sections(&'a self) -> &'a [MenuSection] {
        &self.sections
    }

    pub fn get_dinner_ids(&'a self) -> &'a [DinnerId] {
        &self.dinner_ids
    }

    pub fn get_menu_review_ids(&'a self) -> &'a [MenuReviewId] {
        &self.menu_review_ids
    }
}

impl AggregateRoot for Menu {}

impl Entity for Menu {
    type Id = MenuId;

    fn get_id<'a>(&'a self) -> &'a Self::Id {
        &self.id
    }
}

impl PartialEq for Menu {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
