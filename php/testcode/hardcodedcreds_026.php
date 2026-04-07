<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_encryption_key
function hardcodedcreds026(BenchmarkRequest $req): BenchmarkResponse {
    define('ENC_KEY', 'key1234567890abcd'); // vuln-code-snippet vuln-line php_hardcoded_encryption_key
    $data = $req->param('data');
    $iv = random_bytes(16);
    $ciphertext = openssl_encrypt($data, 'AES-128-CBC', ENC_KEY, 0, $iv);
    return BenchmarkResponse::ok(base64_encode($iv) . ':' . $ciphertext);
}
// vuln-code-snippet end php_hardcoded_encryption_key
