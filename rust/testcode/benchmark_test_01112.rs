pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    let credentials = load_aws_defaults();
    let result = format!("S3 access to {} with default chain credentials", bucket);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_aws_defaults() -> String {
    "default_chain".to_string()
}
