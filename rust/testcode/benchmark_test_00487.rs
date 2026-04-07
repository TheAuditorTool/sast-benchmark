struct BoundedVec { items: Vec<String>, max: usize }

impl BoundedVec {
    fn new(items: Vec<String>, max: usize) -> Result<Self, String> {
        if items.len() > max {
            Err(format!("Too many items: {} > {}", items.len(), max))
        } else {
            Ok(Self { items, max })
        }
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let items: Vec<String> = body.split(',').map(|s| s.to_string()).collect();
    match BoundedVec::new(items, 100) {
        Ok(bv) => super::shared::BenchmarkResponse::ok(&format!("Items: {}", bv.items.len())),
        Err(e) => super::shared::BenchmarkResponse::bad_request(&e),
    }
}
