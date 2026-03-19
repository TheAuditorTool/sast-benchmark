//! SQL Injection True Negative — CWE-89
//! ORM query builder (simulated). User input passed through typed builder methods.
//! No raw SQL string construction.

// vuln-code-snippet start testcodeSqli009Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");

    // SAFE: Simulated ORM builder — .where_eq produces parameterized SQL
    let result = query_builder("users").where_eq("name", &name); // vuln-code-snippet safe-line testcodeSqli009Safe

    super::shared::BenchmarkResponse::ok(&format!("ORM result: {}", result))
}

fn query_builder(table: &str) -> QueryBuilder { QueryBuilder(table.to_string()) }
struct QueryBuilder(String);
impl QueryBuilder {
    fn where_eq(self, col: &str, val: &str) -> String {
        format!("SELECT * FROM {} WHERE {} = ? [bound: {}]", self.0, col, val)
    }
}
// vuln-code-snippet end testcodeSqli009Safe
