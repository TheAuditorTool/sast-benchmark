//! CWE-943: User-supplied role injected inside a $where JS function body — function-level JS injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

// vuln-code-snippet start testcodeNosql018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let role = req.param("role");
    // Attacker sends role="x' || true || 'y" to return all documents.
    let filter = format!(
        "{{\"$where\": \"function() {{ return this.role == '{}' }}\"}}",
        role
    );
    let result = mongo_find("staff", &filter); // vuln-code-snippet target-line testcodeNosql018
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql018
