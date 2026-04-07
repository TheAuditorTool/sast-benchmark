fn session_csrf_token() -> String {
    "stored_token_value".to_string()
}

fn transfer_funds(to: &str, amount: &str) -> bool {
    let _ = (to, amount);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let expected = session_csrf_token();
    if req.header("X-CSRF-Token") == expected {
        let to = req.param("to");
        let amount = req.param("amount");
        let result = transfer_funds(&to, &amount);
        if result {
            return super::shared::BenchmarkResponse::ok("transfer complete");
        }
        return super::shared::BenchmarkResponse::error("transfer failed");
    }
    super::shared::BenchmarkResponse::forbidden("csrf mismatch")
}
