struct Resource {
    owner: String,
    data: String,
}

struct Session {
    user: String,
}

fn db_get_resource(id: &str) -> Option<Resource> {
    Some(Resource {
        owner: "user_456".to_string(),
        data: format!("resource_data_for_{}", id),
    })
}

fn get_session() -> Session {
    Session { user: "user_123".to_string() }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let resource = match db_get_resource(&id) {
        Some(r) => r,
        None => return super::shared::BenchmarkResponse::error("not found"),
    };
    let session = get_session();
    if resource.owner != session.user {
        return super::shared::BenchmarkResponse::forbidden("access denied");
    }
    super::shared::BenchmarkResponse::ok(&resource.data)
}
