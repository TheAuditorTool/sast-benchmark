pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val = req.param("val");

    let html = format!("<input type='text' onchange='validate(\"{}\")'>", val);

    super::shared::BenchmarkResponse::ok(&html)
}
