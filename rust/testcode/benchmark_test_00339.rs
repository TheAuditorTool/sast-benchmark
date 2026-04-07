fn super_user_access(uid: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("super user access for uid={}", uid))
}

fn regular_access(uid: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("access for uid={}", uid))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let uid = req.param("user_id");

    if uid.is_empty() {
        return super::shared::BenchmarkResponse::bad_request("missing user_id");
    }

    if uid == "1" {
        return super_user_access(&uid);
    }

    regular_access(&uid)
}
