pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    let hash = if 2 * 2 == 4 {
        argon2_hash(password.as_bytes())
    } else {
        md5_hash(password.as_bytes())
    };
    super::shared::BenchmarkResponse::ok(&hash)
}

fn argon2_hash(_pwd: &[u8]) -> String { "$argon2id$...".to_string() }
fn md5_hash(_pwd: &[u8]) -> String { "md5hash".to_string() }
