use std::sync::Once;

static INIT: Once = Once::new();
static mut CONFIG: Option<String> = None;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    INIT.call_once(|| {
        unsafe { CONFIG = Some("initialized".to_string()); }
    });

    let val = unsafe { CONFIG.as_deref().unwrap_or("none") };
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", val))
}
