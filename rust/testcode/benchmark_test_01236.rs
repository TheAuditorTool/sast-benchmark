pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");
    let safe = query
        .chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            c => c.to_string(),
        })
        .collect::<String>();
    let html = format!("<p>Showing results for: <strong>{}</strong></p>", safe);
    super::shared::BenchmarkResponse::ok(&html)
}
