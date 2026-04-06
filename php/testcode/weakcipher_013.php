<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_user_cipher
function weakcipher013(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $algo = $req->param('algo');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, $algo, $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_user_cipher
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_user_cipher
