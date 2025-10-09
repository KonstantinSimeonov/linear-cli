use graphql_client::GraphQLQuery;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/create_issue.graphql",
    response_derives = "Debug"
)]
pub struct CreateIssue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/teams.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Teams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/teams.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct TeamMemberships;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/create_issue.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct IssueByIdentifier;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/list.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct MyIssues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/update_issue.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct UpdateIssue;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/linear_schema.graphql",
    query_path = "src/graphql/project.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct Projects;
