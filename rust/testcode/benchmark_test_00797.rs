fn db_get_profile(user_id: &str) -> String {
    format!("profile_data_for_{}", user_id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let profile = db_get_profile(&user_id);
    super::shared::BenchmarkResponse::ok(&profile)
}
