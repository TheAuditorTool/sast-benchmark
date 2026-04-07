<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01019(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $encrypted = openssl_encrypt($data, 'rc4', $key);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
