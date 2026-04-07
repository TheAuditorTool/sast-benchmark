pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    let filtered = filter_printable_ascii(&user_input);
    eprintln!("[INFO] input={}", filtered);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn filter_printable_ascii(input: &str) -> String {
    input.chars().filter(|c| (*c as u32) >= 0x20 && (*c as u32) <= 0x7E).collect()
}
