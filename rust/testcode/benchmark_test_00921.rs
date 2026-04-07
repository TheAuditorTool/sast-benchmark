pub fn handle(_req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut output = String::from("Environment Variables:\n");

    for (key, value) in std::env::vars() {
        output.push_str(&format!("{}={}\n", key, value));
    }

    super::shared::BenchmarkResponse::ok(&output)
}
