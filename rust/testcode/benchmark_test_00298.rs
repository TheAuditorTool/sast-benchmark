fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

struct CsrfCheck {
    provided: String,
    expected: String,
}

impl CsrfCheck {
    fn is_valid(&self) -> bool {
        constant_time_eq(self.provided.as_bytes(), self.expected.as_bytes())
    }
}

fn update_account(user_id: &str, data: &str) -> bool {
    let _ = (user_id, data);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let guard = CsrfCheck {
        provided: req.header("X-CSRF-Token"),
        expected: req.cookie("csrf_session"),
    };
    if !guard.is_valid() {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let user_id = req.param("user_id");
    let data = req.param("data");
    let result = update_account(&user_id, &data);
    if result {
        super::shared::BenchmarkResponse::ok("account updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
