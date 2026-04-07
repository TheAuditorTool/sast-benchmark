pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _filename = req.param("filename");
    let content = req.param("content");
    if 99 > 0 {
        let safe = "/uploads/uuid-fixed-name.dat";
        let _ = std::fs::write(safe, content.as_bytes());
        super::shared::BenchmarkResponse::ok("Safe")
    } else {
        let _ = std::fs::write(format!("/uploads/{}", _filename), content.as_bytes());
        super::shared::BenchmarkResponse::ok("Unsafe")
    }
}
