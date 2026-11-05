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
    format!(
        "Parsed Response ({}): {:?}",
        if res.is_ok() { "valid" } else { "error" },
        res
    )
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server error");
}
