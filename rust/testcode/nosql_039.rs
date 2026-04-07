//! CWE-943: Safe query via validated filter struct that rejects JSON objects on construction.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct SafeFilter {
    value: String,
}

impl SafeFilter {
    fn new(input: &str) -> Result<Self, String> {
        // Reject any input that looks like a JSON object or contains operator characters.
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

// vuln-code-snippet start testcodeNosql039
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sf = match SafeFilter::new(&req.param("q")) {
        Ok(f) => f,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let result = mongo_find("items", &sf.as_filter("name")); // vuln-code-snippet target-line testcodeNosql039
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql039
