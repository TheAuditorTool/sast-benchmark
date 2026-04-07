<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00174(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $cipher = getenv('CIPHER_ALGO') ?: 'aes-256-gcm';
    $iv = random_bytes(12);
    $tag = '';
    $encrypted = openssl_encrypt($data, $cipher, $key, 0, $iv, $tag);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $tag . $encrypted)]);
}
