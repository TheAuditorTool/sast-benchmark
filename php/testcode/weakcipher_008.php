<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_blowfish
function weakcipher008(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $iv = random_bytes(8);
    $encrypted = openssl_encrypt($data, 'bf-cbc', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_blowfish
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_blowfish
