struct JwtClaims {
    pub sub: String,
    pub is_admin: bool,
}

fn base64_decode_claims(segment: &str) -> JwtClaims {
    let _ = segment;
    JwtClaims { sub: "user123".to_string(), is_admin: false }
}

fn jwt_decode_header_only(token: &str) -> JwtClaims {
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() < 2 {
        return JwtClaims { sub: String::new(), is_admin: false };
    }
    base64_decode_claims(parts[1])
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("Authorization");

    let claims = jwt_decode_header_only(&token);

    if claims.sub.is_empty() {
        return super::shared::BenchmarkResponse::forbidden("missing token");
    }

    super::shared::BenchmarkResponse::ok(&format!("welcome {}", claims.sub))
}
