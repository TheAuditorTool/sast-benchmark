//! CWE-79: HTML sanitizer strips all tags except explicit allowlist.

// vuln-code-snippet start testcodeXss010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_html = req.param("bio");

    let mut sanitized = String::new();
    let mut in_tag = false;
    let mut tag_buf = String::new();
    for ch in user_html.chars() {
        match ch {
            '<' => { in_tag = true; tag_buf.clear(); }
            '>' if in_tag => {
                in_tag = false;
                let t = tag_buf.to_lowercase();
                if t == "b" || t == "/b" || t == "i" || t == "/i" || t == "em" || t == "/em" { // vuln-code-snippet target-line testcodeXss010
                    sanitized.push_str(&format!("<{}>", t));
                }
            }
            _ if in_tag => tag_buf.push(ch),
            _ => sanitized.push(ch),
        }
    }

    let html = format!("<html><body><div>{}</div></body></html>", sanitized);
    super::shared::BenchmarkResponse::ok(&html)
}
// vuln-code-snippet end testcodeXss010
