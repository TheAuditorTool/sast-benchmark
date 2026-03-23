<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_jwt_secret
function hardcodedcreds005(BenchmarkRequest $req): BenchmarkResponse {
    $secret = "my-super-secret-jwt-key-2024"; // vuln-code-snippet vuln-line php_hardcoded_jwt_secret
    $payload = json_encode(['user' => $req->param('user'), 'exp' => time() + 3600]);
    $signature = hash_hmac('sha256', $payload, $secret);
    return BenchmarkResponse::json(['token' => base64_encode($payload) . '.' . $signature]);
}
// vuln-code-snippet end php_hardcoded_jwt_secret
