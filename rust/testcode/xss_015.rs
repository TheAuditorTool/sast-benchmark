//! CWE-79: User input stored in memory then rendered without escaping.

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// vuln-code-snippet start testcodeXss015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");

    if action == "save" {
        let name = req.param("name");
        let bio = req.param("bio");
        STORE.lock().unwrap().insert(name, bio);
        return super::shared::BenchmarkResponse::ok("Saved");
    }

    let name = req.param("name");
    let bio = STORE.lock().unwrap()
        .get(&name).cloned().unwrap_or_default();
    let html = format!(
        "<html><body><h1>{}</h1><p>{}</p></body></html>",
        name, bio
    ); // vuln-code-snippet target-line testcodeXss015

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss015
