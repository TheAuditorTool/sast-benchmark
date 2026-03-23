<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_env_file
function hardcodedcreds006(BenchmarkRequest $req): BenchmarkResponse {
    $secret = $_ENV['JWT_SECRET']; // vuln-code-snippet safe-line php_hardcoded_env_file
    $payload = json_encode(['user' => $req->param('user'), 'exp' => time() + 3600]);
    $signature = hash_hmac('sha256', $payload, $secret);
    return BenchmarkResponse::json(['token' => base64_encode($payload) . '.' . $signature]);
}
// vuln-code-snippet end php_hardcoded_env_file
