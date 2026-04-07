pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let item = req.param("item");

    let result = async_mutex_update(&item);
    super::shared::BenchmarkResponse::ok(&format!("Updated: {}", result))
}

fn async_mutex_update(item: &str) -> String {
    format!("locked_and_updated_{}", item)
}
