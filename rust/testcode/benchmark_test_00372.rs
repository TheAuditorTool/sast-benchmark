pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let date_str = req.param("date");
    let date = permissive_date_parse(&date_str);
    super::shared::BenchmarkResponse::ok(&format!("Date: {}", date))
}
fn permissive_date_parse(input: &str) -> String {
    input.to_string()
}
