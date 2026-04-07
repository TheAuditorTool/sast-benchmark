fn export_all_data() -> String {
    "all_user_data_export".to_string()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _format = req.param("format");
    let result = export_all_data();
    super::shared::BenchmarkResponse::ok(&result)
}
