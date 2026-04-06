//! CWE-362: One-time initialization using std::sync::Once.

use std::sync::Once;

// vuln-code-snippet start testcodeRaceCondition016
static INIT: Once = Once::new();
static mut CONFIG: Option<String> = None;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    INIT.call_once(|| { // vuln-code-snippet target-line testcodeRaceCondition016
        unsafe { CONFIG = Some("initialized".to_string()); }
    });

    let val = unsafe { CONFIG.as_deref().unwrap_or("none") };
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", val))
}
// vuln-code-snippet end testcodeRaceCondition016
