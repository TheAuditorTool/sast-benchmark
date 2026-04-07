use std::sync::RwLock;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let new_val = req.param("value");
    static DATA: RwLock<String> = RwLock::new(String::new());
    let mut guard = DATA.write().unwrap();
    *guard = new_val;
    super::shared::BenchmarkResponse::ok("Updated")
}
