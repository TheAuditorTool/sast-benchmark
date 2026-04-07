pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let presigned_url = generate_presigned_url(&filename);
    super::shared::BenchmarkResponse::ok(&format!("Upload to: {}", presigned_url))
}
fn generate_presigned_url(key: &str) -> String {
    format!("https://s3.amazonaws.com/bucket/{}?X-Amz-Signature=abc123", key)
}
