pub trait IDateProvider {
    fn now(&self) -> u64;
}
