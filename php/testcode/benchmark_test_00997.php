<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00997(BenchmarkRequest $req): BenchmarkResponse {
    define('JWT_PUBLIC_KEY', "-----BEGIN PUBLIC KEY-----\nMFwwDQYJKoZIhv...fake...\n-----END PUBLIC KEY-----");
    $token = $req->header('Authorization');
    $parts = explode('.', $token);
    $key = openssl_pkey_get_public(JWT_PUBLIC_KEY);
    $valid = openssl_verify($parts[0] . '.' . $parts[1], base64_decode($parts[2] ?? ''), $key, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::ok($valid === 1 ? 'valid' : 'invalid');
}
