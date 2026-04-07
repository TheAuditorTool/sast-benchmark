pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_page = req.param("page");
    let raw_size = req.param("size");
    let page: u32 = raw_page.parse().unwrap_or(1).max(1);
    let size: u32 = raw_size.parse().unwrap_or(10).min(100);
    let offset = (page - 1) * size;
    let sql = format!("SELECT * FROM items LIMIT {} OFFSET {}", size, offset);
    super::shared::BenchmarkResponse::ok(&sql)
}
