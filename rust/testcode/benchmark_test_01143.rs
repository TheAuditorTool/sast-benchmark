struct ReportRequest {
    start_date: String,
    end_date: String,
    region: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let r = ReportRequest {
        start_date: req.param("start"),
        end_date: req.param("end"),
        region: req.param("region"),
    };
    let sql = format!(
        "SELECT SUM(amount) FROM sales WHERE date BETWEEN '{}' AND '{}' AND region = '{}'",
        r.start_date, r.end_date, r.region
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
