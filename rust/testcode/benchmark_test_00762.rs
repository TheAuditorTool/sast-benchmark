fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    let filter = format!(
        "{{\"$where\": \"function() {{ return this.role == '{}' }}\"}}",
        role
    );
    let result = mongo_find("staff", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
