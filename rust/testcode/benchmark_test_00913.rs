use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

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
    );

    super::shared::BenchmarkResponse::ok(&html)
}
