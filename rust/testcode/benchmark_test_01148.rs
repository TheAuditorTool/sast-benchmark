pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let dept = req.cookie("department");
    let sql = format!(
        "SELECT * FROM employees WHERE name LIKE '{}%' AND department = '{}'",
        name, dept
    );
    super::shared::BenchmarkResponse::ok(&sql)
}
