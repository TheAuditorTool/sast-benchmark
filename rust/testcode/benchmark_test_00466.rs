pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _session = req.param("session");

    let csrf: u32 = rand::random::<u32>();

    super::shared::BenchmarkResponse::ok(&format!("csrf={:08x}", csrf))
}
