use async_graphql::SimpleObject;

#[derive(SimpleObject, Default)]
pub struct A {
    pub a: u8,
}
