fn change_settings(setting: &str) -> bool {
    let _ = setting;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _csrf = req.param("csrf_token");
    let setting = req.param("setting");
    let result = change_settings(&setting);
    if result {
        super::shared::BenchmarkResponse::ok("settings changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
