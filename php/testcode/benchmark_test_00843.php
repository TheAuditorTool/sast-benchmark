<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00843(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = str_repeat("\0", 16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
