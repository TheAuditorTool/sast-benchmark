pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let region = req.param("region");
    let bucket = req.param("bucket");
    let allowed_regions = ["us-east-1", "us-west-2", "eu-west-1"];
    if !allowed_regions.contains(&region.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Invalid region");
    }
    let url = format!("https://s3.{}.amazonaws.com/{}", region, bucket);
    super::shared::BenchmarkResponse::ok(&format!("Accessing: {}", url))
}
