<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_gcm_nonce_stored
function weakcipher050(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $nonce = random_bytes(12);
    $tag = '';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, $nonce, $tag); // vuln-code-snippet safe-line php_weakcipher_gcm_nonce_stored
    $stored = json_encode([
        'nonce' => base64_encode($nonce),
        'tag' => base64_encode($tag),
        'ct' => base64_encode($enc)
    ]);
    return BenchmarkResponse::ok($stored);
}
// vuln-code-snippet end php_weakcipher_gcm_nonce_stored
