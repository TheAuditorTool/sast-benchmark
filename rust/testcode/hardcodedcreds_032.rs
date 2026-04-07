//! CWE-798: Hardcoded Stripe live secret key stored in a const.

// vuln-code-snippet start testcodeHardcodedcreds032
const STRIPE_SECRET_KEY: &str = "sk_live_abc123xyz789stripe"; // vuln-code-snippet target-line testcodeHardcodedcreds032

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount = req.param("amount");
    let result = format!(
        "POST https://api.stripe.com/v1/charges amount={} api_key={}",
        amount, STRIPE_SECRET_KEY
    );
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds032
