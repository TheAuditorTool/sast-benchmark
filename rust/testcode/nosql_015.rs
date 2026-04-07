//! CWE-943: User JSON flows into struct via constructor method then to MongoDB find.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct SearchQuery {
    raw_filter: String,
}

impl SearchQuery {
    fn from_request(req: &super::shared::BenchmarkRequest) -> Self {
        // Constructs filter from req.param — user-supplied JSON is stored verbatim.
        SearchQuery {
            raw_filter: req.param("query"),
        }
    }

    fn execute(&self, collection: &str) -> String {
        mongo_find(collection, &self.raw_filter)
    }
}

// vuln-code-snippet start testcodeNosql015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search = SearchQuery::from_request(req);
    let result = search.execute("inventory"); // vuln-code-snippet target-line testcodeNosql015
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql015
