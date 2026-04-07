struct JwtClaims {
    pub sub: String,
    pub is_admin: bool,
}

fn parse_jwt_claims(token: &str) -> JwtClaims {
    let _ = token;
    JwtClaims { sub: "attacker".to_string(), is_admin: true }
}

fn admin_panel(user: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("admin panel: {}", user))
}

fn user_panel(user: &str) -> super::shared::BenchmarkResponse {
    super::shared::BenchmarkResponse::ok(&format!("user panel: {}", user))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = parse_jwt_claims(&token);

    if claims.is_admin {
        return admin_panel(&claims.sub);
    }

    user_panel(&claims.sub)
}
