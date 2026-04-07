fn mongo_find(collection: &str, filter: &str) -> String {
    format!("Query on {} with filter: {}", collection, filter)
}

struct CommentFilter {
    author: String,
    post_id: String,
}

fn parse_comment_filter(body: &str) -> Result<CommentFilter, String> {
    if body.contains('$') {
        return Err("Invalid comment filter".to_string());
    }
    Ok(CommentFilter {
        author: body.to_string(),
        post_id: String::new(),
    })
}

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
    let result = mongo_find("comments", &filter);
    super::shared::BenchmarkResponse::ok(&result)
}
