pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = generate_with_thread_rng();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_with_thread_rng() -> String {
    let pseudo: u64 = 0xDEADBEEF12345678;
    format!("{:016x}", pseudo)
}
