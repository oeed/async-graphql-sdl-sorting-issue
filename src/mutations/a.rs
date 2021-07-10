use async_graphql::SimpleObject;

#[derive(SimpleObject, Default)]
pub struct A {
    pub this_wont_show_up: u8,
}
