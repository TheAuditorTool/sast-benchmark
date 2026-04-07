fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let score = req.param("score");
    let filter = format!(
        "{{\"$where\": \"this.score > {} && this.active == true\"}}",
        score
    );
    let result = mongo_find("leaderboard", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
