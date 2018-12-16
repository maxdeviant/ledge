use std::io::Cursor;

use juniper_rocket::{graphiql_source, GraphQLRequest, GraphQLResponse};
use rocket::http::ContentType;
use rocket::response::content::Html;
use rocket::{Response, Route};

use crate::database::DatabaseConnection;
use crate::graphql::{Database, Mutation, Query, Schema};

#[get("/graphiql")]
fn graphiql() -> Html<String> {
    graphiql_source("/graphiql")
}

fn execute_graphql_request<'a>(
    connection: DatabaseConnection,
    request: GraphQLRequest,
) -> Response<'a> {
    let context = Database::new(connection);
    let schema = Schema::new(Query, Mutation);
    let GraphQLResponse(status, json) = request.execute(&schema, &context);
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "*")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, GET, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .header(ContentType::new("application", "json"))
        .status(status)
        .sized_body(Cursor::new(json))
        .finalize()
}

#[options("/graphql")]
fn graphql_options_handler<'a>() -> Response<'a> {
    Response::build()
        .raw_header("Access-Control-Allow-Origin", "*")
        .raw_header("Access-Control-Allow-Methods", "OPTIONS, GET, POST")
        .raw_header("Access-Control-Allow-Headers", "Content-Type")
        .finalize()
}

#[get("/graphql?<request>")]
fn graphql_get_handler<'a>(
    connection: DatabaseConnection,
    request: GraphQLRequest,
) -> Response<'a> {
    execute_graphql_request(connection, request)
}

#[post("/graphql", data = "<request>")]
fn graphql_post_handler<'a>(
    connection: DatabaseConnection,
    request: GraphQLRequest,
) -> Response<'a> {
    execute_graphql_request(connection, request)
}

pub fn routes() -> Vec<Route> {
    routes![
        graphiql,
        graphql_options_handler,
        graphql_get_handler,
        graphql_post_handler
    ]
}
