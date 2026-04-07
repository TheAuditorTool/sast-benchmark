fn delete_all_users() -> String {
    "all_users_deleted".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let result = delete_all_users();
    super::shared::BenchmarkResponse::ok(&result)
}
