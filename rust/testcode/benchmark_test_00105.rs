fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

struct SearchQuery {
    raw_filter: String,
}

impl SearchQuery {
    fn from_request(req: &super::shared::BenchmarkRequest) -> Self {
        SearchQuery {
            raw_filter: req.param("query"),
        }
    }

    fn execute(&self, collection: &str) -> String {
        mongo_find(collection, &self.raw_filter)
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let search = SearchQuery::from_request(req);
    let result = search.execute("inventory");
    super::shared::BenchmarkResponse::ok(&result)
}
