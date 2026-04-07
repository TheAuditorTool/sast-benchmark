pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");

    let html = format!(
        "<html><!-- search query: {} --><body><p>Results for your query</p></body></html>",
        query
    );

    super::shared::BenchmarkResponse::ok(&html)
}
