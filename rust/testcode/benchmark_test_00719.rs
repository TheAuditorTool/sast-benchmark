pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.param("content");
    if content.len() > 5_242_880 {
        return super::shared::BenchmarkResponse::bad_request("File too large (max 5MB)");
    }
    let safe_name = format!("{}.dat", generate_uuid());
    let _ = std::fs::write(format!("/uploads/{}", safe_name), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn generate_uuid() -> &'static str { "def456" }
