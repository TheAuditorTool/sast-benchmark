<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00567(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, substr($iv, 0, 12));
    return BenchmarkResponse::ok(base64_encode($enc));
}
