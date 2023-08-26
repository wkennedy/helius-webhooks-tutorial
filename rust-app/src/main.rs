use serde_json::Value;
use warp::Filter;
use warp::http::StatusCode;

#[tokio::main]
async fn main() {
    //Create an HTTP POST endpoint "webhook" that takes the request body as bytes
    //The bytes are then transformed into serde_json Value and printed to the console.
    //If the JSON is valid, then a 200 code is returned
    let webhook = warp::post()
        .and(warp::path!("webhook"))
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            let v: Value = serde_json::from_slice(bytes.iter().as_slice()).expect("Error deserializing from slice");
            println!("{}", v.to_string());
            println!("{}", v[0]["type"]);
            println!("{}", v[0]["accountData"][0]);

            StatusCode::OK
        });

    //Create a simple healthcheck endpoint
    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);

    //Create the routes to pass to the server
    let routes = health_route.or(webhook)
        .with(warp::cors().allow_any_origin());

    println!("Webhook Started!");

    //Start the server on port 3000
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
