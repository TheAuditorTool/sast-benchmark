<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00425(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_aead_chacha20poly1305_ietf_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_AEAD_CHACHA20POLY1305_IETF_NPUBBYTES);
    $enc = sodium_crypto_aead_chacha20poly1305_ietf_encrypt($req->param('data'), '', $nonce, $key);
    return BenchmarkResponse::ok(base64_encode($nonce . $enc));
}
