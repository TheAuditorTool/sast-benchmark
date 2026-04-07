//! CWE-79: Tainted user input stored in a struct field then rendered into HTML without escaping.

// vuln-code-snippet start testcodeXss024
struct Profile {
    name: String,
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let profile = Profile {
        name: req.param("name"),
    };

    let html = format!("<h2 class='user'>{}</h2>", profile.name); // vuln-code-snippet target-line testcodeXss024

    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss024
