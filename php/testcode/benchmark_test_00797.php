<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00797(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(8);
    $encrypted = openssl_encrypt($data, 'bf-cbc', $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
