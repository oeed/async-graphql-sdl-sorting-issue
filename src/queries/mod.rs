mod a;

use a::A;
use async_graphql::MergedObject;

#[derive(Default, MergedObject)]
pub struct Query(A);
