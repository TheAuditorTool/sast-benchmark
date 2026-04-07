const STRIPE_SECRET_KEY: &str = "sk_live_abc123xyz789stripe";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount = req.param("amount");
    let result = format!(
        "POST https://api.stripe.com/v1/charges amount={} api_key={}",
        amount, STRIPE_SECRET_KEY
    );
    super::shared::BenchmarkResponse::ok(&result)
}
