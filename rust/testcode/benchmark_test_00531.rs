pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    let result = query_builder("users").where_eq("name", &name);

    super::shared::BenchmarkResponse::ok(&format!("ORM result: {}", result))
}

fn query_builder(table: &str) -> QueryBuilder { QueryBuilder(table.to_string()) }
struct QueryBuilder(String);
impl QueryBuilder {
    fn where_eq(self, col: &str, val: &str) -> String {
        format!("SELECT * FROM {} WHERE {} = ? [bound: {}]", self.0, col, val)
    }
}
