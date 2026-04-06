//! CWE-1333: Regex compiled once at startup via lazy_static, never from user input.

// vuln-code-snippet start testcodeRedos018
static EMAIL_RE: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    // Simulates: lazy_static! { static ref RE: Regex = Regex::new(EMAIL_RE).unwrap(); }
    let re = compile_once(EMAIL_RE); // vuln-code-snippet target-line testcodeRedos018
    super::shared::BenchmarkResponse::ok(&format!("Valid: {}", re.contains(&email)))
}
fn compile_once(pattern: &str) -> String { format!("compiled({})", pattern) }
// vuln-code-snippet end testcodeRedos018
