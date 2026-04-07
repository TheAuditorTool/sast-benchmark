pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let result = ErrorResult { detail: format!("No row with pk={} in users_table", id) };
    super::shared::BenchmarkResponse::error(&result.detail)
}

struct ErrorResult { detail: String }
