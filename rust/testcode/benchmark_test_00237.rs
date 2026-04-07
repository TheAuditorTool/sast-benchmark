struct StrictRequest { name: String, age: u32 }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let parsed = strict_deser(&json_input);
    super::shared::BenchmarkResponse::ok(&format!("Name: {}", parsed))
}
fn strict_deser(input: &str) -> String {
    format!("strict_parsed(len={})", input.len())
}
