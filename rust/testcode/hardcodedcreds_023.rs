//! CWE-798: Hardcoded AWS access key ID and secret access key in consts.

// vuln-code-snippet start testcodeHardcodedcreds023
const AWS_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
const AWS_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"; // vuln-code-snippet target-line testcodeHardcodedcreds023

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    let result = format!(
        "Accessing S3 bucket {} with key_id={} secret={}",
        bucket, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds023
