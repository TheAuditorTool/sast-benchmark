//! CWE-943: Safe order filter — typed OrderFilter struct deserialization prevents operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct OrderFilter {
    customer_id: String,
    status: String,
}

fn parse_order_filter(body: &str) -> Result<OrderFilter, String> {
    // Simulates typed serde deserialization — String fields reject operator objects.
    if body.contains('$') {
        return Err("Invalid order filter".to_string());
    }
    Ok(OrderFilter {
        customer_id: body.to_string(),
        status: "pending".to_string(),
    })
}

// vuln-code-snippet start testcodeNosql034
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let of = match parse_order_filter(&body) {
        Ok(f) => f,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let filter = format!(
        "{{\"customer_id\":{{\"$eq\":\"{}\"}},\"status\":{{\"$eq\":\"{}\"}}}}",
        of.customer_id.replace('"', "\\\""),
        of.status.replace('"', "\\\"")
    );
    let result = mongo_find("orders", &filter); // vuln-code-snippet target-line testcodeNosql034
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql034
