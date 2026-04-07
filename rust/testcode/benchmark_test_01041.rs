fn get_session_uid() -> String {
    "user_123".to_string()
}

fn db_get_permissions(user_id: &str) -> Vec<String> {
    if user_id == "admin_001" {
        vec!["read".to_string(), "write".to_string(), "export".to_string()]
    } else {
        vec!["read".to_string()]
    }
}

fn export_data() -> String {
    "export_complete".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _format = req.param("format");
    let session_uid = get_session_uid();
    let permissions = db_get_permissions(&session_uid);
    if !permissions.contains(&"export".to_string()) {
        return super::shared::BenchmarkResponse::forbidden("export permission required");
    }
    let result = export_data();
    super::shared::BenchmarkResponse::ok(&result)
}
