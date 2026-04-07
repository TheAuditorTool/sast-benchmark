fn admin_dashboard(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin dashboard for {}", username))
}

fn user_dashboard(username: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user dashboard for {}", username))
}

fn get_session_user(session: &str) -> Option<String> {
    if session.is_empty() { None } else { Some("alice".to_string()) }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session = req.cookie("session");

    let username = match get_session_user(&session) {
        Some(u) => u,
        None => return super::shared::BenchmarkResponse::forbidden("not logged in"),
    };

    if req.param("role") == "admin" {
        return admin_dashboard(&username);
    }

    user_dashboard(&username)
}
