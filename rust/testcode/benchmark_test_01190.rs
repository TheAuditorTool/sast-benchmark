use std::fs;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("uid");
    let filename = req.param("file");
    let path = format!("/home/{}/documents/{}", user_id, filename);
    match fs::remove_file(&path) {
        Ok(_) => super::shared::BenchmarkResponse::ok("Deleted"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
