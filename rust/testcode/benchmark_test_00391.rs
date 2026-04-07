fn db_get_invoice_owned(id: &str, user_id: &str) -> Option<String> {
    if id == "inv_1" && user_id == "user_123" {
        Some(format!("invoice_data_for_{}", id))
    } else {
        None
    }
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("invoice_id");
    let session_uid = get_session_user_id();
    match db_get_invoice_owned(&id, &session_uid) {
        Some(invoice) => super::shared::BenchmarkResponse::ok(&invoice),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
