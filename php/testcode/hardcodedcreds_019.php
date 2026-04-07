<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_jwt_constructor
function hardcodedcreds019(BenchmarkRequest $req): BenchmarkResponse {
    $jwtSecret = 'my-hardcoded-jwt-secret-key'; // vuln-code-snippet vuln-line php_hardcoded_jwt_constructor
    $token = $req->header('Authorization');
    $parts = explode('.', $token);
    $sig = hash_hmac('sha256', $parts[0] . '.' . $parts[1], $jwtSecret, true);
    $valid = isset($parts[2]) && hash_equals(base64_encode($sig), $parts[2]);
    return BenchmarkResponse::ok($valid ? 'valid' : 'invalid');
}
// vuln-code-snippet end php_hardcoded_jwt_constructor
