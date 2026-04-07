fn delete_record(id: &str) -> String {
    format!("record_{}_deleted", id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let is_admin = req.cookie("admin") == "true";
    if is_admin {
        let result = delete_record(&id);
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
