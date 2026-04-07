<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01032(BenchmarkRequest $req): BenchmarkResponse {
    $jwtSecret = 'my-hardcoded-jwt-secret-key';
    $token = $req->header('Authorization');
    $parts = explode('.', $token);
    $sig = hash_hmac('sha256', $parts[0] . '.' . $parts[1], $jwtSecret, true);
    $valid = isset($parts[2]) && hash_equals(base64_encode($sig), $parts[2]);
    return BenchmarkResponse::ok($valid ? 'valid' : 'invalid');
}
