<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_rsa_public_key_only
function hardcodedcreds038(BenchmarkRequest $req): BenchmarkResponse {
    define('JWT_PUBLIC_KEY', "-----BEGIN PUBLIC KEY-----\nMFwwDQYJKoZIhv...fake...\n-----END PUBLIC KEY-----"); // vuln-code-snippet safe-line php_hardcoded_rsa_public_key_only
    $token = $req->header('Authorization');
    $parts = explode('.', $token);
    $key = openssl_pkey_get_public(JWT_PUBLIC_KEY);
    $valid = openssl_verify($parts[0] . '.' . $parts[1], base64_decode($parts[2] ?? ''), $key, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::ok($valid === 1 ? 'valid' : 'invalid');
}
// vuln-code-snippet end php_hardcoded_rsa_public_key_only
