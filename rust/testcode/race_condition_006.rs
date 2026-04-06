//! CWE-362: Global singleton initialized without synchronization primitive.

use std::sync::Mutex;

// vuln-code-snippet start testcodeRaceCondition006
static GLOBAL: Mutex<Option<String>> = Mutex::new(None);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let config = req.param("config");

    if GLOBAL.lock().unwrap().is_none() { // vuln-code-snippet target-line testcodeRaceCondition006
        // Lock dropped; another thread may also see None and initialize
        *GLOBAL.lock().unwrap() = Some(config.clone());
    }

    let val = GLOBAL.lock().unwrap().clone().unwrap_or_default();
    super::shared::BenchmarkResponse::ok(&format!("Config: {}", val))
}
// vuln-code-snippet end testcodeRaceCondition006
