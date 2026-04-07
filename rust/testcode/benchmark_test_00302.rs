fn mongo_aggregate(collection: &str, pipeline_stage: &str) -> String {
    format!("Aggregate on {} with stage: {}", collection, pipeline_stage)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let stage = format!("{{\"$match\": {}}}", req.param("criteria"));
    let result = mongo_aggregate("products", &stage);
    super::shared::BenchmarkResponse::ok(&result)
}
