<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01031(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(12);
    $tag = '';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, $iv, $tag);
    $result = base64_encode($iv . $tag . $enc);
    return BenchmarkResponse::ok($result);
}
