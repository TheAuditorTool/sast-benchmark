pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let next = req.param("next");
    let allowed_return = get_allowed_return_url();
    if next == allowed_return {
        let location = format!("Location: {}", next);
        super::shared::BenchmarkResponse::ok(&location)
    } else {
        super::shared::BenchmarkResponse::bad_request("Unexpected redirect")
    }
}

fn get_allowed_return_url() -> String {
    "/dashboard".to_string()
}
