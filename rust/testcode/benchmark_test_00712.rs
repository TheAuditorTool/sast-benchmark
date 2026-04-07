use regex::RegexSet;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let text = req.param("text");
    let set = RegexSet::new(&[
        r"^\d{4}-\d{2}-\d{2}$",
        r"^[a-f0-9]{32}$",
        r"^[A-Z]{2}\d{6}$",
    ]).unwrap();
    let matches: Vec<_> = set.matches(&text).into_iter().collect();
    super::shared::BenchmarkResponse::ok(&format!("Matched patterns: {:?}", matches))
}
