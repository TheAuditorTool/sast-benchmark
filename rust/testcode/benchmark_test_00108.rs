pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_token();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_token() -> String {
    let pid: u64 = 1234;
    let val = pid.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
    format!("{:016x}", val)
}
