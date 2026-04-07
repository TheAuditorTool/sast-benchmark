//! CWE-943: Safe comment filter — typed CommentFilter struct deserialization prevents operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct CommentFilter {
    author: String,
    post_id: String,
}

fn parse_comment_filter(body: &str) -> Result<CommentFilter, String> {
    // Simulates typed serde deserialization — String fields reject operator objects.
    if body.contains('$') {
        return Err("Invalid comment filter".to_string());
    }
    Ok(CommentFilter {
        author: body.to_string(),
        post_id: String::new(),
    })
}

// vuln-code-snippet start testcodeNosql035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let cf = match parse_comment_filter(&body) {
        Ok(f) => f,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let filter = format!(
        "{{\"author\":{{\"$eq\":\"{}\"}},\"post_id\":{{\"$eq\":\"{}\"}}}}",
        cf.author.replace('"', "\\\""),
        cf.post_id.replace('"', "\\\"")
    );
    let result = mongo_find("comments", &filter); // vuln-code-snippet target-line testcodeNosql035
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql035
