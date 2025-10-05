use crate::client::get_client;
use graphql_client::{GraphQLQuery, Response, Error};
use reqwest::blocking::Client;

pub fn gql_request<Q: GraphQLQuery>(
    variables: Q::Variables,
) -> Result<Q::ResponseData, Vec<Error>> {
    let client: Client = get_client();

    let res = graphql_client::reqwest::post_graphql_blocking::<Q, _>(
        &client,
        "https://api.linear.app/graphql",
        variables,
    ).unwrap();

    match res {
      Response { errors: Some(errors), .. } => Result::Err(errors),
      Response { data: Some(data), .. } => Result::Ok(data),
      _ => panic!()
    }
}
