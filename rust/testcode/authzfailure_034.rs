//! CWE-285: Permission enum matched exhaustively; privileged action only for Admin variant

enum Permission {
    Admin,
    User,
    Guest,
}

fn get_session_permission() -> Permission {
    Permission::User
}

fn perform_action() -> String {
    "privileged_action_executed".to_string()
}

fn forbidden_response() -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::forbidden("insufficient permissions")
}

// vuln-code-snippet start testcodeAuthzfailure034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _data = req.param("data");
    let user_permission = get_session_permission();
    match user_permission {
        Permission::Admin => {
            let result = perform_action(); // vuln-code-snippet target-line testcodeAuthzfailure034
            super::shared::BenchmarkResponse::ok(&result)
        }
        _ => forbidden_response(),
    }
}
// vuln-code-snippet end testcodeAuthzfailure034
