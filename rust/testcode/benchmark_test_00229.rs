struct TokenConfig {
    seed: u64,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let cfg = TokenConfig { seed: 1_700_000_000 };
    let token = generate_from_seed(cfg.seed);

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn generate_from_seed(seed: u64) -> String {
    let val = seed.wrapping_mul(2_862_933_555_777_941_757).wrapping_add(3_037_000_493);
    format!("{:016x}", val)
}
