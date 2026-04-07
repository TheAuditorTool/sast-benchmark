fn access_granted(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("access granted: {}", username))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");

    if req.cookie("authenticated") == "true" {
        return access_granted(&username);
    }

    super::shared::BenchmarkResponse::forbidden("not authenticated")
}
