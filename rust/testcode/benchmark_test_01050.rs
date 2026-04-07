pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(-1);
    let username = req.param("user");
    match ValidatedForm::new(age, username) {
        Some(form) => {
            let result = process_form(&form);
            super::shared::BenchmarkResponse::ok(&result)
        }
        None => super::shared::BenchmarkResponse::bad_request("Validation failed"),
    }
}

struct ValidatedForm { age: i32, username: String }
impl ValidatedForm {
    fn new(age: i32, username: String) -> Option<Self> {
        if (1..=150).contains(&age) && username.len() >= 3 {
            Some(ValidatedForm { age, username })
        } else { None }
    }
}
fn process_form(f: &ValidatedForm) -> String { format!("user={} age={}", f.username, f.age) }
