<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_des
function weakcipher001(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $encrypted = openssl_encrypt($data, 'des-ecb', $key); // vuln-code-snippet vuln-line php_weakcipher_des
    return BenchmarkResponse::json(['ciphertext' => base64_encode($encrypted)]);
}
// vuln-code-snippet end php_weakcipher_des
