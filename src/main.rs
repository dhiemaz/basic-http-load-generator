use hyper::{Body, Method, Client, Request};


#[tokio::main]
async fn main() {

    // create http client
    let client = Client::builder().
        pool_idle_timeout(Some(std::time::Duration::from_secs(5))).
        http2_only(true).
        build_http();

    // initialize request
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://httpbin.org/get")
        .body(Body::empty())
        .unwrap();

    // send request
    let future = client.request(req);

    future.

    println!("Hello, world!");
}
