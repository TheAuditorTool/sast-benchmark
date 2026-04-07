pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");
    let msg = req.cookie("msg");
    let html = format!(
        "<div class='search'>Results for: {}</div><p class='notice'>{}</p>",
        query, msg
    );
    super::shared::BenchmarkResponse::ok(&html)
}
