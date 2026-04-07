pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let city = req.param("city");
    let sql = format!(
        "SELECT hotel_name, price FROM hotels WHERE city = '{}' AND available = 1",
        city
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
