struct AuthContext {
    pub user_id: u64,
    pub verified: bool,
}

fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    let _ = (password, hash);
    Ok(password == "correct")
}

fn get_user_id_and_hash(username: &str) -> Result<(u64, String), String> {
    if username.is_empty() {
        return Err("user not found".to_string());
    }
    Ok((7u64, "$2b$12$fakehash".to_string()))
}

fn serve_protected(ctx: &AuthContext) -> super::shared::BenchmarkResponse {
    if !ctx.verified {
        return super::shared::BenchmarkResponse::forbidden("not verified");
    }
    super::shared::BenchmarkResponse::ok(&format!("resource for uid={}", ctx.user_id))
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    let password = req.param("password");

    let (user_id, hash) = match get_user_id_and_hash(&username) {
        Ok(pair) => pair,
        Err(e) => return super::shared::BenchmarkResponse::forbidden(&e),
    };

    let verified = match bcrypt_verify(&password, &hash) {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::error("hash error"),
    };

    let ctx = AuthContext { user_id, verified };

    serve_protected(&ctx)
}
