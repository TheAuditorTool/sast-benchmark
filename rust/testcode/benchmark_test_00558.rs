pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let route_name = req.param("route");
    let target = match route_name.as_str() {
        "home" => "/",
        "profile" => "/user/profile",
        "settings" => "/user/settings",
        "logout" => "/auth/logout",
        _ => "/",
    };
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
