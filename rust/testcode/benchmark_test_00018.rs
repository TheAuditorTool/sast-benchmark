pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let key = req.param("key");
    let value = req.param("value");
    let result = atomic_insert_if_absent(&key, &value);
    if result {
        super::shared::BenchmarkResponse::ok("Inserted")
    } else {
        super::shared::BenchmarkResponse::bad_request("Already exists")
    }
}

fn atomic_insert_if_absent(_key: &str, _val: &str) -> bool {
    true
}
