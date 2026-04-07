pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redirect_id = req.param("id");
    match lookup_redirect(&redirect_id) {
        Some(url) => super::shared::BenchmarkResponse::ok(&format!("Location: {}", url)),
        None => super::shared::BenchmarkResponse::bad_request("Unknown redirect ID"),
    }
}
fn lookup_redirect(id: &str) -> Option<String> {
    match id { "1" => Some("/home".into()), "2" => Some("/profile".into()), _ => None }
}
