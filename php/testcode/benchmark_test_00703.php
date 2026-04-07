<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00703(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = 'ThisIsAHardcodedEncryptionKey123';
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
