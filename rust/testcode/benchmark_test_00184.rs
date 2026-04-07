use std::sync::Mutex;

static GLOBAL: Mutex<Option<String>> = Mutex::new(None);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let config = req.param("config");

    if GLOBAL.lock().unwrap().is_none() {
        *GLOBAL.lock().unwrap() = Some(config.clone());
    }

    let val = GLOBAL.lock().unwrap().clone().unwrap_or_default();
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", val))
}
