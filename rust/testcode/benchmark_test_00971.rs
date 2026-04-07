fn get_session_role(session_token: &str) -> String {
    let _ = session_token;
    "user".to_string()
}

fn privileged_action() -> String {
    "sensitive_operation_result".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let session_token = req.cookie("session");
    let role = get_session_role(&session_token);
    if role != "admin" {
        return super::shared::BenchmarkResponse::forbidden("insufficient privileges");
    }
    let result = privileged_action();
    super::shared::BenchmarkResponse::ok(&result)
}
