fn post_comment(comment: &str) -> bool {
    let _ = comment;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("csrf") != "" {
        let comment = req.param("comment");
        let result = post_comment(&comment);
        if result {
            return super::shared::BenchmarkResponse::ok("comment posted");
        }
    }
    super::shared::BenchmarkResponse::bad_request("missing csrf token")
}
