const ADMIN_PASSWORD: &str = "Admin@123!";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    if password == ADMIN_PASSWORD {
        super::shared::BenchmarkResponse::ok("Login successful")
    } else {
        super::shared::BenchmarkResponse::forbidden("Invalid password")
    }
}
