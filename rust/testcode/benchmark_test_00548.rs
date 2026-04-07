fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

struct SearchOpts {
    filter: String,
}

impl SearchOpts {
    fn from_request(filter_json: &str) -> Self {
        SearchOpts {
            filter: filter_json.to_string(),
        }
    }

    fn as_query(&self) -> &str {
        &self.filter
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let opts = SearchOpts::from_request(&req.param("opts"));
    let result = mongo_find("catalog", opts.as_query());
    super::shared::BenchmarkResponse::ok(&result)
}
