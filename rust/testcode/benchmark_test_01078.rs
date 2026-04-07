fn superadmin_panel() -> String {
    "superadmin_panel_data".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.param("superadmin") == "1" {
        let result = superadmin_panel();
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
