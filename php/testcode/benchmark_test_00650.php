<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00650(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = sodium_crypto_secretbox_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
    $encrypted = sodium_crypto_secretbox($data, $nonce, $key);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($nonce . $encrypted)]);
}
