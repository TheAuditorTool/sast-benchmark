<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_pbkdf2_key
function weakcipher016(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $password = getenv('ENCRYPTION_PASSWORD');
    $salt = random_bytes(16);
    $key = hash_pbkdf2('sha256', $password, $salt, 100000, 32, true);
    $iv = random_bytes(12);
    $tag = '';
    $encrypted = openssl_encrypt($data, 'aes-256-gcm', $key, 0, $iv, $tag); // vuln-code-snippet safe-line php_weakcipher_pbkdf2_key
    return BenchmarkResponse::json(['ciphertext' => base64_encode($salt . $iv . $tag . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_pbkdf2_key
