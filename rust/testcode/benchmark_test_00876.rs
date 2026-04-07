fn check_permission(_resource_id: &str, _user_id: &str) -> bool {
    false
}

fn db_get_resource(id: &str) -> String {
    format!("resource_data_for_{}", id)
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let session_user_id = get_session_user_id();
    let _ = check_permission(&id, &session_user_id);
    let resource = db_get_resource(&id);
    super::shared::BenchmarkResponse::ok(&resource)
}
