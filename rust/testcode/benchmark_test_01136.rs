pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let keyword = req.cookie("search");
    let sql = format!("SELECT title, body FROM posts WHERE title LIKE '%{}%'", keyword);
    super::shared::BenchmarkResponse::ok(&format!("Results for: {}", sql))
}
