use domain::common::errors::{Error, ValidationError};
use inflections::case::{to_pascal_case, to_title_case};
use validator::Validate;

pub mod interfaces;

pub fn validate(validator: impl Validate) -> std::result::Result<(), Error> {
    validator.validate().map_err(|error| {
        let error = error.field_errors().into_iter().fold(
            ValidationError::default(),
            |error, (field, errors)| {
                let messages = errors
                    .iter()
                    .map(|err| format!("'{}' {}", to_title_case(field), err))
                    .collect();

                error.add_error(&to_pascal_case(field), messages)
            },
        );

        Error::from(error)
    })
}
