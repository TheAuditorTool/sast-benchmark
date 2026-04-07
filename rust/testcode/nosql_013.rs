//! CWE-943: User JSON tainted through a struct field before reaching MongoDB find.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct SearchOpts {
    filter: String,
}

impl SearchOpts {
    fn from_request(filter_json: &str) -> Self {
        // Simulates: serde_json::from_str::<serde_json::Value>(filter_json)
        // Tainted JSON is stored without validation — operators survive struct construction.
        SearchOpts {
            filter: filter_json.to_string(),
        }
    }

    fn as_query(&self) -> &str {
        &self.filter
    }
}

// vuln-code-snippet start testcodeNosql013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let opts = SearchOpts::from_request(&req.param("opts"));
    let result = mongo_find("catalog", opts.as_query()); // vuln-code-snippet target-line testcodeNosql013
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql013
