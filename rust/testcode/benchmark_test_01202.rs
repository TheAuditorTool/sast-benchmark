pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let import_src = req.header("X-Import-Source");
    let token = req.header("X-Auth-Token");
    super::shared::BenchmarkResponse::ok(&format!("Importing data from {} with token {}", import_src, token))
}
