use axum::{
    Json, Router,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sypher::parser::query::Query;

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckQueryRequest {
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckQueryResponse {
    pub result: String,
}

fn parse_query(input: &str) -> String {
    println!("Received query: '{input}'");
    let query = Query::from_str(input);
    let res = sypher::parser::parse_query::parse_query(query);
    if res.is_ok() {
        format!("Your query's syntax is valid. Here is the Debug-representation of the generated IR:\n\n{:?}", res)
    } else {
        format!("There is a syntactic error in the query:\n\n{:?}", res)
    }
}

async fn check_query(
    Json(query_req): Json<CheckQueryRequest>,
) -> (StatusCode, Json<CheckQueryResponse>) {
    let query = query_req.query;
    let parser_result = parse_query(&query);
    let res = CheckQueryResponse {
        result: parser_result,
    };
    (StatusCode::OK, Json(res))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/check", post(check_query))
        .route("/", get(index));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(
        format!("0.0.0.0:{}", port)
    )
    .await
    .expect("failed to bind");

    println!("Listening on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await.expect("server error");
}
