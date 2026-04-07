<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01092(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, '');
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
