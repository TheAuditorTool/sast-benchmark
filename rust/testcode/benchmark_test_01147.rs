pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let zip = req.param("zip");
    if zip.len() != 5 {
        return super::shared::BenchmarkResponse::bad_request("Zip must be 5 digits");
    }
    let sql = format!("SELECT city, state FROM zipcodes WHERE zip = '{}'", zip);
    super::shared::BenchmarkResponse::ok(&sql)
}
