//! CWE-943: Aggregation pipeline $match stage built from user-controlled JSON input.

fn mongo_aggregate(collection: &str, pipeline_stage: &str) -> String {
    // In production: mongodb::Collection::aggregate(pipeline, None).await
    format!("Aggregate on {} with stage: {}", collection, pipeline_stage)
}

struct Pipeline {
    match_stage: String,
}

impl Pipeline {
    fn from_match(criteria: &str) -> Self {
        // Simulates building a $match stage from raw user JSON.
        // No operator validation — {"$match": {"price": {"$gt": 0}}} is accepted unchanged.
        Pipeline {
            match_stage: format!("{{\"$match\": {}}}", criteria),
        }
    }
}

// vuln-code-snippet start testcodeNosql011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pipeline = Pipeline::from_match(&req.param("match"));
    let result = mongo_aggregate("orders", &pipeline.match_stage); // vuln-code-snippet target-line testcodeNosql011
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql011
