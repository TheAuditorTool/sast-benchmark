fn privileged_op() -> String {
    "privileged_operation_executed".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let level = req.header("X-Permission-Level").parse::<u32>().unwrap_or(0);
    if level >= 10 {
        let result = privileged_op();
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("insufficient permission level")
}
