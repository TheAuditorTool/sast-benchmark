<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_rsa_private_key
function hardcodedcreds027(BenchmarkRequest $req): BenchmarkResponse {
    $privateKey = "-----BEGIN EC PRIVATE KEY-----\nMHQCAQEEI...fake...\n-----END EC PRIVATE KEY-----"; // vuln-code-snippet vuln-line php_hardcoded_rsa_private_key
    $data = $req->bodyStr();
    $key = openssl_pkey_get_private($privateKey);
    openssl_sign($data, $signature, $key, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::ok(base64_encode($signature));
}
// vuln-code-snippet end php_hardcoded_rsa_private_key
