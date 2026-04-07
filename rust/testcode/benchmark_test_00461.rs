pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    if id.is_empty() {
        return super::shared::BenchmarkResponse::bad_request(
            r#"{"error":"missing_id","status":400}"#,
        );
    }

    let body = r#"{"error":"not_found","status":404}"#;

    super::shared::BenchmarkResponse { status: 404, body: body.to_string() }
}
