//! CWE-434: Pre-signed S3 URL generated for client-side upload, server never touches file.

// vuln-code-snippet start testcodeFileupload014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let presigned_url = generate_presigned_url(&filename); // vuln-code-snippet target-line testcodeFileupload014
    super::shared::BenchmarkResponse::ok(&format!("Upload to: {}", presigned_url))
}
fn generate_presigned_url(key: &str) -> String {
    // Simulates: s3_client.generate_presigned_url(PutObject, bucket, key, 300)
    format!("https://s3.amazonaws.com/bucket/{}?X-Amz-Signature=abc123", key)
}
// vuln-code-snippet end testcodeFileupload014
