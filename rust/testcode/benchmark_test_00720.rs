pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");
    let src = input.as_bytes();

    if src.len() > 256 {
        return super::shared::BenchmarkResponse::bad_request("Input too large");
    }

    let mut dst = vec![0u8; src.len()];

    dst.copy_from_slice(src);

    super::shared::BenchmarkResponse::ok(&format!("Copied {} bytes", dst.len()))
}
