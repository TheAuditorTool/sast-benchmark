pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("message");

    let result = channel_send(&message);
    super::shared::BenchmarkResponse::ok(&format!("Sent: {}", result))
}

fn channel_send(msg: &str) -> String {
    format!("channel_msg_{}", msg)
}
