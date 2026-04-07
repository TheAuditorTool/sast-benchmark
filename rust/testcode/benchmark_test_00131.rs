fn fetch_profile(user_id: &str) -> String {
    let _ = user_id;
    "profile_data".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let user_data = fetch_profile(&user_id);
    super::shared::BenchmarkResponse::ok(&user_data)
}
