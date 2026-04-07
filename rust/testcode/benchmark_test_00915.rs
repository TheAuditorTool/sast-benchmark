fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

fn session_get_csrf_token(session_id: &str) -> String {
    let _ = session_id;
    "session_stored_token".to_string()
}

struct CsrfGuard {
    session_token: String,
}

impl CsrfGuard {
    fn from_session(session_id: &str) -> Self {
        CsrfGuard {
            session_token: session_get_csrf_token(session_id),
        }
    }

    fn validate(&self, provided: &str) -> bool {
        constant_time_eq(provided.as_bytes(), self.session_token.as_bytes())
    }
}

fn update_user_data(user_id: &str, data: &str) -> bool {
    let _ = (user_id, data);
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_id = req.cookie("session_id");
    let guard = CsrfGuard::from_session(&session_id);
    let provided_token = req.header("X-CSRF-Token");
    if !guard.validate(&provided_token) {
        return super::shared::BenchmarkResponse::forbidden("csrf validation failed");
    }
    let user_id = req.param("user_id");
    let data = req.param("data");
    let result = update_user_data(&user_id, &data);
    if result {
        super::shared::BenchmarkResponse::ok("user data updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
