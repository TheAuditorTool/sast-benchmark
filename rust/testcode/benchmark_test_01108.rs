fn update_data(data: &str) -> bool {
    let _ = data;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("csrf");
    if token.len() > 0 {
    }
    let data = req.param("data");
    let result = update_data(&data);
    if result {
        super::shared::BenchmarkResponse::ok("data updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
