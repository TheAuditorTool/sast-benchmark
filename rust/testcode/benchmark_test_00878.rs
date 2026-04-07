use std::str::FromStr;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_uri = req.param("uri");

    let uri = match http::Uri::from_str(&user_uri) {
        Ok(u) => u,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };

    let request = http::Request::builder()
        .method("GET")
        .uri(uri)
        .body(())
        .unwrap();

    let _resp = hyper_client_request(request);

    super::shared::BenchmarkResponse::ok("Request dispatched")
}

fn hyper_client_request(req: http::Request<()>) -> String {
    format!("Response from {}", req.uri())
}
