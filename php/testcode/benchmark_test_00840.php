<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00840(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(8);
    $encrypted = openssl_encrypt($data, 'des-ede3-cbc', $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
