pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            let msg = safe_error_msg(&e);
            super::shared::BenchmarkResponse::error(msg)
        }
    }
}

fn safe_error_msg(_internal_err: &str) -> &'static str { "An error occurred" }
fn db_query(_id: &str) -> Result<String, String> { Ok("data".to_string()) }
