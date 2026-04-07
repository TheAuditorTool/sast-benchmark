<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00476(BenchmarkRequest $req): BenchmarkResponse {
    define('ENC_KEY', 'key1234567890abcd');
    $data = $req->param('data');
    $iv = random_bytes(16);
    $ciphertext = openssl_encrypt($data, 'AES-128-CBC', ENC_KEY, 0, $iv);
    return BenchmarkResponse::ok(base64_encode($iv) . ':' . $ciphertext);
}
