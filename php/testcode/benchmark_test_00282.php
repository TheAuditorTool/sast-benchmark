<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00282(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(12);
    $tag = '';
    $encrypted = openssl_encrypt($data, 'chacha20-poly1305', $key, 0, $iv, $tag);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $tag . $encrypted)]);
}
