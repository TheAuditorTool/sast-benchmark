pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("id");

    let parsed: Result<i64, _> = user_id.parse();
    match parsed {
        Ok(id) => {
            let html = format!("<html><body><p>User #{}</p></body></html>", id);
            super::shared::BenchmarkResponse::ok(&html)
        }
        Err(_) => {
            let html = format!(
                "<html><body><p>Invalid user ID: {}</p></body></html>",
                user_id
            );
            super::shared::BenchmarkResponse::ok(&html)
        }
    }
}
