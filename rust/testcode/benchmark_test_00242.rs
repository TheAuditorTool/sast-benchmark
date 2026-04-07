pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let result = match action.as_str() {
        "create" | "read" | "update" | "delete" => format!("Action: {}", action),
        _ => "Unknown action".to_string(),
    };
    super::shared::BenchmarkResponse::ok(&result)
}
