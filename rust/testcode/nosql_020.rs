//! CWE-943: User-supplied score interpolated into $where JS expression — numeric boundary injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let score = req.param("score");
    // Attacker sends score="0 || 1==1" making the expression always true.
    let filter = format!(
        "{{\"$where\": \"this.score > {} && this.active == true\"}}",
        score
    );
    let result = mongo_find("leaderboard", &filter); // vuln-code-snippet target-line testcodeNosql020
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql020
