fn add_admin_role(user_id: &str) -> bool {
    let _ = user_id;
    true
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let result = add_admin_role(&user_id);
    if result {
        super::shared::BenchmarkResponse::ok("admin role assigned")
    } else {
        super::shared::BenchmarkResponse::error("role assignment failed")
    }
}
