use md5;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let digest = format!("{:x}", md5::compute(password.as_bytes()));
    super::shared::BenchmarkResponse::ok(&format!("Hash: {}", digest))
}
