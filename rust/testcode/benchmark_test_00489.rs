pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("id");
    match execute_sql(&user_id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => super::shared::BenchmarkResponse::error(&e),
    }
}

fn execute_sql(_id: &str) -> Result<String, String> {
    Err("ERROR: column \"secret_salary\" of table \"employees\" does not exist".to_string())
}
