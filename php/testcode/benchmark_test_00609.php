<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00609(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $algo = $req->param('algo');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, $algo, $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
