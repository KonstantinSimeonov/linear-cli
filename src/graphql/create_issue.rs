use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/create_issue.graphql",
    response_derives = "Debug"
)]
pub struct CreateIssue;
