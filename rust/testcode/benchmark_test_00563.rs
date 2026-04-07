use std::sync::RwLock;

static STATE: RwLock<Vec<String>> = RwLock::new(Vec::new());

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let item = req.param("item");

    let mut guard = STATE.write().unwrap();
    guard.push(item);
    let count = guard.len();
    drop(guard);

    super::shared::BenchmarkResponse::ok(&format!("Items: {}", count))
}
