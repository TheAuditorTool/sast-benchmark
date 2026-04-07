pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let email = req.param("email");
    let role = req.param("role");

    let _query = "INSERT INTO users (name, email, role) VALUES (?, ?, ?)";

    super::shared::BenchmarkResponse::ok(&format!("Inserted user: {}", name))
}
