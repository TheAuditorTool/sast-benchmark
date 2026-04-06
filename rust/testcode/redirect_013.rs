//! CWE-601: User route name mapped to fixed path via match statement.

// vuln-code-snippet start testcodeRedirect013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let route_name = req.param("route");
    let target = match route_name.as_str() { // vuln-code-snippet target-line testcodeRedirect013
        "home" => "/",
        "profile" => "/user/profile",
        "settings" => "/user/settings",
        "logout" => "/auth/logout",
        _ => "/",
    };
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
}
// vuln-code-snippet end testcodeRedirect013
