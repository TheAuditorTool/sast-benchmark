pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let status = req.param("status");
    let allowed_statuses = ["pending", "active", "closed", "archived"];
    if !allowed_statuses.contains(&status.as_str()) {
        return super::shared::BenchmarkResponse::bad_request("Invalid status");
    }
    let sql = format!("SELECT * FROM tickets WHERE status = '{}'", status);
    super::shared::BenchmarkResponse::ok(&sql)
}
