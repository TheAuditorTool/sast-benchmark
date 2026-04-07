pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(_e) => super::shared::BenchmarkResponse::error("Internal error. Request ID: req-12345"),
    }
}

fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
