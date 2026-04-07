<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_stripe_live_key
function hardcodedcreds018(BenchmarkRequest $req): BenchmarkResponse {
    $stripe = new stdClass();
    $stripe->apiKey = 'sk_live_realKeyHere123456abcdef'; // vuln-code-snippet vuln-line php_hardcoded_stripe_live_key
    $amount = (int)$req->param('amount');
    $charge = ['amount' => $amount, 'currency' => 'usd'];
    return BenchmarkResponse::json($charge);
}
// vuln-code-snippet end php_hardcoded_stripe_live_key
