pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let info = get_server_info();
    super::shared::BenchmarkResponse::ok(&info)
}

fn get_server_info() -> String {
    "/opt/app/bin/server v1.2.3 built 2024-01-15 on host build-server-01".to_string()
}
