//! CWE-943: User controls both field name and value in query — operator injection on either axis.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

fn build_query_from_params(field: &str, value: &str) -> String {
    // Simulates: doc! { field: bson::from_str(value) }
    // Both the key and value come from user input — attacker can supply
    // field="$where" value="function(){return true}" or value={"$gt":""}
    format!("{{\"{}\": {}}}", field, value)
}

// vuln-code-snippet start testcodeNosql004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filter = build_query_from_params(&req.param("field"), &req.param("value"));
    let result = mongo_find("records", &filter); // vuln-code-snippet target-line testcodeNosql004
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql004
