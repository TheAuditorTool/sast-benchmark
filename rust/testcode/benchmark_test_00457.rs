pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_url = req.param("url");

    let response = ureq::get(&user_url).call();

    match response {
        Ok(resp) => {
            let body = resp.into_string().unwrap_or_default();
            super::shared::BenchmarkResponse::ok(&format!("Fetched: {}", body))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
