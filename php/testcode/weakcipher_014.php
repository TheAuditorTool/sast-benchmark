<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_chacha20
function weakcipher014(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(12);
    $tag = '';
    $encrypted = openssl_encrypt($data, 'chacha20-poly1305', $key, 0, $iv, $tag); // vuln-code-snippet safe-line php_weakcipher_chacha20
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $tag . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_chacha20
