//! CWE-798: AWS credentials resolved via SDK default credential chain at runtime.

// vuln-code-snippet start testcodeHardcodedcreds013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    // Simulates: aws_config::load_defaults(BehaviorVersion::latest()).await
    let credentials = load_aws_defaults(); // vuln-code-snippet target-line testcodeHardcodedcreds013
    let result = format!("S3 access to {} with default chain credentials", bucket);
    super::shared::BenchmarkResponse::ok(&result)
}

fn load_aws_defaults() -> String {
    // Simulates: aws_config::load_defaults(BehaviorVersion::latest()).await
    "default_chain".to_string()
}
// vuln-code-snippet end testcodeHardcodedcreds013
