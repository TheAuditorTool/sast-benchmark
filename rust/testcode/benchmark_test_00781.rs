use std::sync::Mutex;

static COUNTER: Mutex<u64> = Mutex::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let mut guard = COUNTER.lock().unwrap();
    *guard += 1;
    let val = *guard;
    drop(guard);

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val))
}
