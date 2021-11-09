#[derive(Debug, GraphQLObject)]
#[graphql(description = "")]
pub struct User {
    pub id: String,
    pub name: String,
}
