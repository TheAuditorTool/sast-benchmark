fn mongo_aggregate(collection: &str, pipeline_stage: &str) -> String {
    format!("Aggregate on {} with stage: {}", collection, pipeline_stage)
}

struct Pipeline {
    match_stage: String,
}

impl Pipeline {
    fn from_match(criteria: &str) -> Self {
        Pipeline {
            match_stage: format!("{{\"$match\": {}}}", criteria),
        }
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let pipeline = Pipeline::from_match(&req.param("match"));
    let result = mongo_aggregate("orders", &pipeline.match_stage);
    super::shared::BenchmarkResponse::ok(&result)
}
