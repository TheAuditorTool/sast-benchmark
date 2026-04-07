pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let users = list_all_users();
    super::shared::BenchmarkResponse::ok(&users)
}

fn list_all_users() -> String {
    "alice,bob,charlie,admin".to_string()
}
