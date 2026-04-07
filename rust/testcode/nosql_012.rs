//! CWE-943: Aggregation $match stage assembled with format! from user-supplied criteria string.

fn mongo_aggregate(collection: &str, pipeline_stage: &str) -> String {
    // In production: mongodb::Collection::aggregate(pipeline, None).await
    format!("Aggregate on {} with stage: {}", collection, pipeline_stage)
}

// vuln-code-snippet start testcodeNosql012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    // Attacker supplies criteria={"price":{"$gt":0},"$where":"function(){return true}"}
    let stage = format!("{{\"$match\": {}}}", req.param("criteria"));
    let result = mongo_aggregate("products", &stage); // vuln-code-snippet target-line testcodeNosql012
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql012
