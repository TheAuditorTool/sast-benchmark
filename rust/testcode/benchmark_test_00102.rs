pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let form = UserForm { age: req.param("age").parse().unwrap_or(0), username: req.param("user") };
    let result = process_form(&form);
    super::shared::BenchmarkResponse::ok(&result)
}

struct UserForm { age: i32, username: String }
fn process_form(f: &UserForm) -> String { format!("user={} age={}", f.username, f.age) }
