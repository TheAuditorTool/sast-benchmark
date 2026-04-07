pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provider = req.param("provider");
    let url = match provider.as_str() {
        "github" => "https://api.github.com/users",
        "gitlab" => "https://gitlab.com/api/v4/users",
        "bitbucket" => "https://api.bitbucket.org/2.0/users",
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown provider"),
    };
    super::shared::BenchmarkResponse::ok(&format!("Fetching from: {}", url))
}
