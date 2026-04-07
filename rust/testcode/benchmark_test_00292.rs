fn get_session_user_id() -> String {
    "user_123".to_string()
}

fn get_authorized_resource(id: &str, session_user_id: &str) -> Result<String, String> {
    if id == "res_1" && session_user_id == "user_123" {
        Ok(format!("resource_content_for_{}", id))
    } else {
        Err("not found or access denied".to_string())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_uid = get_session_user_id();
    match get_authorized_resource(&id, &session_uid) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::forbidden(&e),
    }
}
