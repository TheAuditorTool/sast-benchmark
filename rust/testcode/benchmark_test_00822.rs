use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let file_id = req.param("id");

    let mut file_map: HashMap<&str, &str> = HashMap::new();
    file_map.insert("1", "/var/data/report.txt");
    file_map.insert("2", "/var/data/summary.csv");

    let path = match file_map.get(file_id.as_str()) {
        Some(p) => *p,
        None => return super::shared::BenchmarkResponse::bad_request("File not found"),
    };

    match std::fs::read_to_string(path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
