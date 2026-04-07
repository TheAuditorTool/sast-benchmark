fn post_message(msg: &str) -> bool {
    let _ = msg;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.cookie("csrf") == req.header("X-CSRF-Token") {
        let msg = req.param("message");
        let result = post_message(&msg);
        if result {
            return super::shared::BenchmarkResponse::ok("message posted");
        }
        return super::shared::BenchmarkResponse::error("post failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
