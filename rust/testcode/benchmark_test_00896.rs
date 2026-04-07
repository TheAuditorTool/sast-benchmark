fn unauthenticated_fetch(id: &str) -> String {
    format!("unauthenticated_data_for_{}", id)
}

fn authorized_fetch(id: &str, session_uid: &str) -> String {
    format!("authorized_data_for_{}_{}", id, session_uid)
}

fn get_session_uid() -> String {
    "user_123".to_string()
}

const ENFORCE_AUTH: bool = true;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_uid();
    let result = if !ENFORCE_AUTH {
        unauthenticated_fetch(&id)
    } else {
        authorized_fetch(&id, &session_uid)
    };
    super::shared::BenchmarkResponse::ok(&result)
}
