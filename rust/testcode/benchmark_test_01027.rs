const AWS_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
const AWS_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let bucket = req.param("bucket");
    let result = format!(
        "Accessing S3 bucket {} with key_id={} secret={}",
        bucket, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    );
    super::shared::BenchmarkResponse::ok(&result)
}
