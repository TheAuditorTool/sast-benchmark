pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let group_name = req.header("X-Group");
    let sql = format!(
        "SELECT u.id, u.name FROM users u JOIN groups g ON u.group_id = g.id WHERE g.name = '{}'",
        group_name
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
