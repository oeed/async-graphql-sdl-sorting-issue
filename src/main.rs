mod mutations;
mod queries;

use mutations::Mutation;
use queries::Query;

fn main() {
    let schema = async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        async_graphql::types::EmptySubscription::default(),
    )
    .finish();

    println!("{}", schema.sdl());
}
