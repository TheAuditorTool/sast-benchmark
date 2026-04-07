fn delete_resource(id: &str) -> bool {
    let _ = id;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    if !csrf.starts_with("csrf_") {
        return super::shared::BenchmarkResponse::bad_request("invalid csrf token format");
    }
    let id = req.param("id");
    let result = delete_resource(&id);
    if result {
        super::shared::BenchmarkResponse::ok("resource deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
