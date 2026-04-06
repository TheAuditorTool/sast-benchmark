//! CWE-434: Image re-processed through image crate to strip embedded payloads.

// vuln-code-snippet start testcodeFileupload017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    let clean_image = reprocess_image(content.as_bytes()); // vuln-code-snippet target-line testcodeFileupload017
    let _ = std::fs::write("uploads/clean.png", &clean_image);
    super::shared::BenchmarkResponse::ok("Image re-processed and saved")
}
fn reprocess_image(data: &[u8]) -> Vec<u8> {
    // Simulates: image::load_from_memory(data)?.save_with_format(path, ImageFormat::Png)
    data.to_vec()
}
// vuln-code-snippet end testcodeFileupload017
