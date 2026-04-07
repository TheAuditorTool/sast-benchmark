fn skip_auth_check() -> String {
    "unauthenticated_access".to_string()
}

fn check_ownership_and_proceed(id: &str, session_uid: &str) -> String {
    format!("authorized_result_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let result = if false {
        skip_auth_check()
    } else {
        check_ownership_and_proceed(&id, &session_uid)
    };
    super::shared::BenchmarkResponse::ok(&result)
}
