use serde_json::Value;
use warp::Filter;
use warp::http::StatusCode;

#[tokio::main]
async fn main() {
    let webhook = warp::post()
        .and(warp::path!("webhook"))
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            let v: Value = serde_json::from_slice(bytes.iter().as_slice()).expect("Error deserializing from slice");
            println!("{}", v.to_string());
            StatusCode::OK
        });

    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);

    let routes = health_route.or(webhook)
        .with(warp::cors().allow_any_origin());

    println!("Webhook Started!");

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
