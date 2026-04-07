<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00361(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $encrypted = openssl_encrypt($data, 'aes-128-ecb', $key);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
