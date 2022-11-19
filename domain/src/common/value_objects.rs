pub use average_rating::*;
pub use rating::*;

mod average_rating {
    use crate::common::models::ValueObject;

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct AverageRating {
        value: f32,
        num_ratings: u8,
    }

    impl AverageRating {
        pub fn new(value: f32, num_ratings: u8) -> Self {
            Self { value, num_ratings }
        }

        pub fn get_num_ratings(&self) -> u8 {
            self.num_ratings
        }

        pub fn add_new_rating(&mut self, rating: super::Rating) {
            let Self { value, num_ratings } = AverageRatingOps::Add.calculate(&self, &rating);

            self.num_ratings = num_ratings;
            self.value = value;
        }

        pub fn remove_rating(&mut self, rating: super::Rating) {
            let Self { value, num_ratings } = AverageRatingOps::Sub.calculate(&self, &rating);

            self.num_ratings = num_ratings;
            self.value = value;
        }
    }

    impl<'a> ValueObject<'a> for AverageRating {
        type Value = f32;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.value
        }
    }

    enum AverageRatingOps {
        Add,
        Sub,
    }

    impl AverageRatingOps {
        fn calculate(
            &self,
            average_rating: &AverageRating,
            rating: &super::Rating,
        ) -> AverageRating {
            let num_ratings = match self {
                AverageRatingOps::Add => average_rating.num_ratings + 1,
                AverageRatingOps::Sub => average_rating.num_ratings - 1,
            };

            let rating_value = rating.get_value();
            let multi = average_rating.value * to_f32(average_rating.num_ratings);

            let mut new_value = match self {
                AverageRatingOps::Add => multi + rating_value,
                AverageRatingOps::Sub => multi - rating_value,
            };

            new_value /= to_f32(num_ratings);

            AverageRating::new(new_value, num_ratings)
        }
    }

    fn to_f32(num_ratings: u8) -> f32 {
        f32::try_from(num_ratings).unwrap()
    }
}

mod rating {
    use crate::common::models::ValueObject;

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Rating(f32);

    impl Rating {
        pub fn new(value: f32) -> Self {
            Self(value)
        }
    }

    impl<'a> ValueObject<'a> for Rating {
        type Value = f32;

        fn get_value(&'a self) -> &'a Self::Value {
            &self.0
        }
    }
}
