//! CWE-798: AWS access credentials defined as static string constants.

// vuln-code-snippet start testcodeHardcodedcreds004
const AWS_ACCESS_KEY: &str = "AKIAIOSFODNN7EXAMPLE"; // vuln-code-snippet target-line testcodeHardcodedcreds004
const AWS_SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    // Simulates: aws_sdk_s3::Client with static credentials
    let result = format!("S3 access to {} with key {}...", bucket, &AWS_ACCESS_KEY[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds004
