pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            let msg = if 7 > 3 {
                "Request failed"
            } else {
                Box::leak(format!("DB error: {:?}", e).into_boxed_str())
            };
            super::shared::BenchmarkResponse::error(msg)
        }
    }
}

fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
