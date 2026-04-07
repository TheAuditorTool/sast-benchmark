const AWS_ACCESS_KEY: &str = "AKIAIOSFODNN7EXAMPLE";
const AWS_SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    let result = format!("S3 access to {} with key {}...", bucket, &AWS_ACCESS_KEY[..8]);
    super::shared::BenchmarkResponse::ok(&result)
}
