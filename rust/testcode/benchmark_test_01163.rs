pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_year = req.param("year");
    let year: u32 = match raw_year.parse::<u32>() {
        Ok(y) if y >= 2000 && y <= 2100 => y,
        _ => return super::shared::BenchmarkResponse::bad_request("Invalid year"),
    };
    let sql = format!("SELECT month, SUM(revenue) FROM sales WHERE year = {} GROUP BY month", year);
    super::shared::BenchmarkResponse::ok(&sql)
}
