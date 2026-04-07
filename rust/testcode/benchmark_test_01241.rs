use serde::Deserialize;

#[derive(Deserialize)]
struct UserProfile {
    name: String,
    role: String,
    permissions: Vec<String>,
    metadata: std::collections::HashMap<String, serde_json::Value>,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let body = req.body_str();
    let profile: UserProfile = match serde_json::from_str(&body) {
        Ok(p) => p,
        Err(e) => return super::shared::BenchmarkResponse::bad_request(&e.to_string()),
    };
    super::shared::BenchmarkResponse::ok(&format!("Profile: {} role={} perms={:?}", profile.name, profile.role, profile.permissions))
}
