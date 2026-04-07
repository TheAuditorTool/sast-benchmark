pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let sku = req.param("sku");
    let warehouse = req.header("X-Warehouse");
    let sql = format!(
        "SELECT quantity FROM inventory WHERE sku = '{}' AND warehouse_id = '{}'",
        sku, warehouse
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
