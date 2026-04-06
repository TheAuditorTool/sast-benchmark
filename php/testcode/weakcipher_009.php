<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_hardcoded_iv
function weakcipher009(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = str_repeat("\0", 16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_hardcoded_iv
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
// vuln-code-snippet end php_weakcipher_hardcoded_iv
