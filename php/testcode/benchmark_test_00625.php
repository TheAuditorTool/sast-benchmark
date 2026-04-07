<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00625(BenchmarkRequest $req): BenchmarkResponse {
    $privateKey = "-----BEGIN EC PRIVATE KEY-----\nMHQCAQEEI...fake...\n-----END EC PRIVATE KEY-----";
    $data = $req->bodyStr();
    $key = openssl_pkey_get_private($privateKey);
    openssl_sign($data, $signature, $key, OPENSSL_ALGO_SHA256);
    return BenchmarkResponse::ok(base64_encode($signature));
}
