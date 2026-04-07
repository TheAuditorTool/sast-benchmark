pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redirect_uri = req.param("redirect_uri");
    let client_id = req.param("client_id");
    if is_registered_redirect(&client_id, &redirect_uri) {
        let location = format!("Location: {}?code=xyz", redirect_uri);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Unregistered redirect_uri")
    }
}

fn is_registered_redirect(client_id: &str, uri: &str) -> bool {
    let _ = client_id;
    uri == "https://example.com/callback"
}
