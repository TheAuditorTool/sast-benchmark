//! CWE-117: User action mapped to predefined enum variant before logging.

// vuln-code-snippet start testcodeLoginjection013
enum UserAction { Login, Logout, Search, Update, Unknown }

impl std::fmt::Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserAction::Login => write!(f, "LOGIN"),
            UserAction::Logout => write!(f, "LOGOUT"),
            UserAction::Search => write!(f, "SEARCH"),
            UserAction::Update => write!(f, "UPDATE"),
            UserAction::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action_str = req.param("action");

    let action = match action_str.as_str() { // vuln-code-snippet target-line testcodeLoginjection013
        "login" => UserAction::Login,
        "logout" => UserAction::Logout,
        "search" => UserAction::Search,
        "update" => UserAction::Update,
        _ => UserAction::Unknown,
    };

    eprintln!("[AUDIT] action={}", action);
    super::shared::BenchmarkResponse::ok(&format!("Action: {}", action))
}
// vuln-code-snippet end testcodeLoginjection013
