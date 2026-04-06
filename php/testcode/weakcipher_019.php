<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_env_cipher
function weakcipher019(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $cipher = getenv('CIPHER_ALGO') ?: 'aes-256-gcm';
    $iv = random_bytes(12);
    $tag = '';
    $encrypted = openssl_encrypt($data, $cipher, $key, 0, $iv, $tag); // vuln-code-snippet safe-line php_weakcipher_env_cipher
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $tag . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_env_cipher
