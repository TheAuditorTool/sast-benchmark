pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");

    let html = format!(
        "<html><body><button onclick='doAction(\"{}\")'>Go</button></body></html>",
        action
    );

    super::shared::BenchmarkResponse::ok(&html)
}
