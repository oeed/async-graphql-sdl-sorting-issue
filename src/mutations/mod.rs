use a::A;
use async_graphql::MergedObject;

mod a;

#[derive(Default, MergedObject)]
pub struct Mutation(A);
