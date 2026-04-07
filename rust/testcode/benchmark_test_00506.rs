pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index: usize = req.param("dest").parse().unwrap_or(usize::MAX);
    let destinations = ["/home", "/profile", "/settings", "/dashboard"];
    match destinations.get(index) {
        Some(dest) => {
            let location = format!("Location: {}", dest);
            super::shared::BenchmarkResponse::ok(&location)
        }
        None => super::shared::BenchmarkResponse::bad_request("Invalid destination"),
    }
}
