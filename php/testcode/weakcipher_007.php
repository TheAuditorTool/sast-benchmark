<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_3des
function weakcipher007(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(8);
    $encrypted = openssl_encrypt($data, 'des-ede3-cbc', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_3des
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_3des
