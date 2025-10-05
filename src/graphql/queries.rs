use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/create_issue.graphql",
    response_derives = "Debug"
)]
pub struct CreateIssue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",   // path to Linear schema you downloaded
    query_path = "src/graphql/teams.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Teams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",   // path to Linear schema you downloaded
    query_path = "src/graphql/teams.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct TeamMemberships;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",   // path to Linear schema you downloaded
    query_path = "src/graphql/create_issue.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct IssueByIdentifier;
