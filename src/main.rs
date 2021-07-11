use async_graphql::{MergedObject, SimpleObject, Object, InputObject};

#[derive(InputObject)]
pub struct SomeInput {
    input_a: bool
}

#[derive(SimpleObject, Default)]
pub struct A {
    pub a: u8,
}

#[derive(Default, MergedObject)]
pub struct Query(A);


#[derive(Default)]
pub struct B;

#[Object]
impl B {
    pub async fn a(&self, _input: SomeInput) -> bool { true }
}


#[derive(Default, MergedObject)]
pub struct Mutation(B);

fn main() {
    let schema = async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        async_graphql::types::EmptySubscription::default(),
    )
    .finish();

    println!("{}", schema.sdl());
}
