<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00141(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $nonce = random_bytes(12);
    $tag = '';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, $nonce, $tag);
    $stored = json_encode([
        'nonce' => base64_encode($nonce),
        'tag' => base64_encode($tag),
        'ct' => base64_encode($enc)
    ]);
    return BenchmarkResponse::ok($stored);
}
