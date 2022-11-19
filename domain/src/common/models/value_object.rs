pub trait ValueObject<'a>: Clone + PartialEq {
    type Value;

    fn get_value(&'a self) -> &'a Self::Value;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Price {
    amount: f32,
    currency: Currency,
}

impl Price {
    pub fn new(amount: f32, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn get_amount(&self) -> f32 {
        self.amount
    }

    pub fn get_currency<'a>(&'a self) -> &'a Currency {
        &self.currency
    }
}

impl<'a> ValueObject<'a> for Price {
    type Value = ();

    fn get_value(&'a self) -> &'a Self::Value {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Currency {}
