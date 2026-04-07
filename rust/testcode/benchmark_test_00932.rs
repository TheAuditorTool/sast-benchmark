fn superuser_op() -> String {
    "superuser_operation_executed".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    if role == "superuser" {
        let result = superuser_op();
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
