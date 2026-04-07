fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

struct SafeFilter {
    value: String,
}

impl SafeFilter {
    fn new(input: &str) -> Result<Self, String> {
        if input.starts_with('{') || input.starts_with('[') || input.contains('$') {
            return Err("structured input not allowed".to_string());
        }
        Ok(SafeFilter {
            value: input.replace('"', "\\\""),
        })
    }

    fn as_filter(&self, field: &str) -> String {
        format!("{{\"{}\":{{\"$eq\":\"{}\"}}}}", field, self.value)
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sf = match SafeFilter::new(&req.param("q")) {
        Ok(f) => f,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let result = mongo_find("items", &sf.as_filter("name"));
    super::shared::BenchmarkResponse::ok(&result)
}
