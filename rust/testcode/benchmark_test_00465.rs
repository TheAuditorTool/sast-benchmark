struct ImageFetch {
    src: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let fetch = ImageFetch {
        src: req.param("image_url"),
    };

    let _resp = simulated_reqwest_get(&fetch.src);

    super::shared::BenchmarkResponse::ok("Image fetched")
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
