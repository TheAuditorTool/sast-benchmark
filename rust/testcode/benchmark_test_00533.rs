use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let env_vars: HashMap<String, String> = std::env::vars().collect();
    let json = format!("{:?}", env_vars);
    super::shared::BenchmarkResponse::ok(&json)
}
