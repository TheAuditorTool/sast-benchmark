struct ApiClient {
    base_url: &'static str,
}

impl ApiClient {
    fn fetch_item(&self, id: &str) -> String {
        let url = format!("{}/items/{}", self.base_url, id);
        simulated_reqwest_get(&url)
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let item_id = req.param("id");

    if !item_id.chars().all(|c| c.is_ascii_digit()) {
        return super::shared::BenchmarkResponse::bad_request("ID must be numeric");
    }

    let client = ApiClient { base_url: "https://api.example.com" };

    let _resp = client.fetch_item(&item_id);

    super::shared::BenchmarkResponse::ok(&format!("Fetched item: {}", item_id))
}

fn simulated_reqwest_get(url: &str) -> String {
    format!("Response from {}", url)
}
