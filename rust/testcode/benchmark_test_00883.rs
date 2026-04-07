fn db_get_medical_record(id: &str) -> String {
    format!("medical_record_data_for_{}", id)
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("record_id");
    let record = db_get_medical_record(&id);
    super::shared::BenchmarkResponse::ok(&record)
}
