extern crate core;

use aws_sdk_s3::Endpoint;
use axum::http::Uri;
use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // configure your cors setting
    // let cors_layer = CorsLayer::permissive();

    // the aws credentials from environment
    let aws_configuration = aws_config::load_from_env().await;

    let bucket_uri = Uri::from_static("http://localhost:9000");
    let bucket_endpoint = Endpoint::immutable(bucket_uri);
    let config = aws_sdk_s3::config::Builder::from(&aws_configuration)
        .endpoint_resolver(bucket_endpoint)
        .build();

    //create aws s3 client
    let aws_s3_client = aws_sdk_s3::Client::from_conf(config);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/registry", post(upload))
        // set your cors config
        // .layer(cors_layer)
        // pass the aws s3 client to route handler
        .layer(Extension(aws_s3_client));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload(
    Extension(s3_client): Extension<aws_sdk_s3::Client>,
    mut files: Multipart,
) -> Result<String, StatusCode> {
    while let Some(file) = files.next_field().await.unwrap() {
        let bytes = file.bytes().await.map_err(|err| {
            dbg!(err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let resp = s3_client
            .put_object()
            .bucket("test")
            .key("key")
            .body(bytes.into())
            .send()
            .await
            .map_err(|err| {
                dbg!(err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        dbg!(resp);
    }

    Ok(String::from("Uploaded"))
}
