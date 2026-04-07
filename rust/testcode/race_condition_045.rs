//! CWE-362: Condition variable used with Mutex-protected predicate to avoid spurious wakeup race.

use std::sync::{Condvar, Mutex};

// vuln-code-snippet start testcodeRaceCondition045
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _key = req.param("key");
    let pair = get_condvar_pair();
    let (lock, _cvar) = &*pair;
    let guard = lock.lock().unwrap();
    let _guard = _cvar.wait_while(guard, |ready| !*ready).unwrap(); // vuln-code-snippet target-line testcodeRaceCondition045
    super::shared::BenchmarkResponse::ok("Resource ready")
}

fn get_condvar_pair() -> std::sync::Arc<(Mutex<bool>, Condvar)> {
    std::sync::Arc::new((Mutex::new(true), Condvar::new()))
}
// vuln-code-snippet end testcodeRaceCondition045
