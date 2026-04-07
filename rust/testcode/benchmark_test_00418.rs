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

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _data = req.param("data");
    let user_permission = get_session_permission();
    match user_permission {
        Permission::Admin => {
            let result = perform_action();
            super::shared::BenchmarkResponse::ok(&result)
        }
        _ => forbidden_response(),
    }
}
