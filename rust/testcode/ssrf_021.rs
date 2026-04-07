//! CWE-918: Image source URL from form field flows through struct into fetch call.

// vuln-code-snippet start testcodeSsrf021
struct ImageFetch {
    src: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let fetch = ImageFetch {
        src: req.param("image_url"),
    };

    let _resp = simulated_reqwest_get(&fetch.src); // vuln-code-snippet target-line testcodeSsrf021

    super::shared::BenchmarkResponse::ok("Image fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    // In production: reqwest::get(url).await
    format!("Response from {}", url)
}
// vuln-code-snippet end testcodeSsrf021
