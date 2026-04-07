fn update_settings(settings: &str) -> bool {
    let _ = settings;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf_raw = req.param("csrf");
    let _ = csrf_raw.parse::<u64>();
    let settings = req.param("settings");
    let result = update_settings(&settings);
    if result {
        super::shared::BenchmarkResponse::ok("settings updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
