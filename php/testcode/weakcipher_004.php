<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_cbc_iv
function weakcipher004(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv); // vuln-code-snippet safe-line php_weakcipher_cbc_iv
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_cbc_iv
