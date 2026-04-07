enum RequestType { Create, Update, Delete }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();
    let request_type = tagged_enum_deser(&json_input);
    super::shared::BenchmarkResponse::ok(&format!("Type: {}", request_type))
}
fn tagged_enum_deser(input: &str) -> String {
    if input.contains("Create") { "Create".into() }
    else if input.contains("Update") { "Update".into() }
    else if input.contains("Delete") { "Delete".into() }
    else { "rejected_unknown_variant".into() }
}
