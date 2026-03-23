<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_api_key
function hardcodedcreds003(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = "sk_live_abc123def456"; // vuln-code-snippet vuln-line php_hardcoded_api_key
    $ch = curl_init('https://api.example.com/charge');
    curl_setopt($ch, CURLOPT_HTTPHEADER, ['Authorization: Bearer ' . $apiKey]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_hardcoded_api_key
