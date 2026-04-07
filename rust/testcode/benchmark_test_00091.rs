pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let allowed = ["PATH", "LANG", "TZ", "HOME"];
    let mut output = String::from("Environment:\n");

    for key in &allowed {
        if let Ok(val) = std::env::var(key) {
            output.push_str(&format!("{}={}\n", key, val));
        }
    }

    super::shared::BenchmarkResponse::ok(&output)
}
