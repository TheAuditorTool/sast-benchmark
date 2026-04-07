pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let provided = req.param("password");
    let admin_password = "Adm1n$ecure2024!";
    if provided == admin_password {
        super::shared::BenchmarkResponse::ok("Access granted")
    } else {
        super::shared::BenchmarkResponse::forbidden("Access denied")
    }
}
