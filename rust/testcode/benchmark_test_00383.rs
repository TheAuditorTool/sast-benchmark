enum Route { Home, Profile, Settings, Help }

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let route_str = req.param("route");
    let route = match route_str.as_str() {
        "home" => Route::Home,
        "profile" => Route::Profile,
        "settings" => Route::Settings,
        "help" => Route::Help,
        _ => return super::shared::BenchmarkResponse::bad_request("Unknown route"),
    };
    let path = match route {
        Route::Home => "/", Route::Profile => "/profile",
        Route::Settings => "/settings", Route::Help => "/help",
    };
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", path))
}
