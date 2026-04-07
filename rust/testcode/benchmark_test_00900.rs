pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let debug = req.param("debug");
    match db_query(&id) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            if debug == "true" {
                super::shared::BenchmarkResponse::error(&format!("Detail: {:?}", e))
            } else {
                super::shared::BenchmarkResponse::error("Internal error")
            }
        }
    }
}

fn db_query(_id: &str) -> Result<String, String> { Err("column X not found".to_string()) }
