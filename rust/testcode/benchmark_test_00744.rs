struct UserRequest {
    username: String,
    age: u32,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let json_input = req.body_str();

    let user: UserRequest = typed_parse(&json_input);

    super::shared::BenchmarkResponse::ok(&format!("User: {} age {}", user.username, user.age))
}

fn typed_parse(input: &str) -> UserRequest {
    UserRequest { username: input.to_string(), age: 0 }
}
