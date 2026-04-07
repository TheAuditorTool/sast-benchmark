pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    match fetch_record(&id) {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(e) => {
            eprintln!("[ERROR] DB error for id={}: {}", id, e);
            super::shared::BenchmarkResponse::error("Internal error")
        }
    }
}
fn fetch_record(id: &str) -> Result<String, String> {
    if id.is_empty() { Err("connection refused at 10.0.1.5:5432".into()) } else { Ok(format!("record_{}", id)) }
}
