//! CWE-943: Safe product filter — typed struct deserialization blocks operator injection.

fn mongo_find(collection: &str, filter: &str) -> String {
    // In production: mongodb::Collection::find(filter, None).await
    format!("Query on {} with filter: {}", collection, filter)
}

struct ProductFilter {
    name: String,
    category: String,
}

fn parse_product_filter(body: &str) -> Result<ProductFilter, String> {
    // Simulates typed serde deserialization — {"$ne": ""} can't deserialize as String.
    if body.contains('$') {
        return Err("Invalid filter".to_string());
    }
    Ok(ProductFilter {
        name: body.to_string(),
        category: String::new(),
    })
}

// vuln-code-snippet start testcodeNosql032
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.param("body");
    let pf = match parse_product_filter(&body) {
        Ok(f) => f,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e),
    };
    let filter = format!(
        "{{\"name\":{{\"$eq\":\"{}\"}},\"category\":{{\"$eq\":\"{}\"}}}}",
        pf.name.replace('"', "\\\""),
        pf.category.replace('"', "\\\"")
    );
    let result = mongo_find("products", &filter); // vuln-code-snippet target-line testcodeNosql032
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeNosql032
