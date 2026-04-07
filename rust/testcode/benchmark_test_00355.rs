pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let internal_ip = get_internal_ip();
    let msg = format!("Server: {} processed your request", internal_ip);
    super::shared::BenchmarkResponse::ok(&msg)
}

fn get_internal_ip() -> &'static str { "192.168.1.100" }
