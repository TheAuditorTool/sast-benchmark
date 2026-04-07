use std::sync::Mutex;

static COUNTER: Mutex<u64> = Mutex::new(0);

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");

    let val = *COUNTER.lock().unwrap();
    *COUNTER.lock().unwrap() = val + 1;

    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val + 1))
}
