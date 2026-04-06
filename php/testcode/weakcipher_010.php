<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_hardcoded_key
function weakcipher010(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = 'ThisIsAHardcodedEncryptionKey123';
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_hardcoded_key
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_hardcoded_key
