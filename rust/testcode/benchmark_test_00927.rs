pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let new_email = req.param("email");
    if user_exists(&user_id) {
        update_user_email(&user_id, &new_email);
        super::shared::BenchmarkResponse::ok("Updated")
    } else {
        super::shared::BenchmarkResponse::bad_request("User not found")
    }
}

fn user_exists(_id: &str) -> bool { true }
fn update_user_email(_id: &str, _email: &str) {}
