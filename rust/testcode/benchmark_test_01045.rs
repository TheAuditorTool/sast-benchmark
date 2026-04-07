struct Profile {
    name: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let profile = Profile {
        name: req.param("name"),
    };

    let html = format!("<h2 class='user'>{}</h2>", profile.name);

    super::shared::BenchmarkResponse::ok(&html)
}
