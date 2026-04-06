<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_null_iv
function weakcipher012(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, ''); // vuln-code-snippet vuln-line php_weakcipher_null_iv
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
// vuln-code-snippet end php_weakcipher_null_iv
