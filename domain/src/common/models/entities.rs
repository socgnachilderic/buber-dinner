pub trait Entity: PartialEq {
    type Id: ?Sized;

    fn get_id<'a>(&'a self) -> &'a Self::Id;
}
