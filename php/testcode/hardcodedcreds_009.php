<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_base64_key
function hardcodedcreds009(BenchmarkRequest $req): BenchmarkResponse {
    $key = base64_decode('c3VwZXJfc2VjcmV0X2tleV8xMjM0NTY3OA=='); // vuln-code-snippet vuln-line php_hardcoded_base64_key
    $data = $req->post('data');
    $iv = random_bytes(16);
    $encrypted = openssl_encrypt($data, 'aes-256-cbc', $key, 0, $iv);
    return BenchmarkResponse::json(['ciphertext' => base64_encode($iv . $encrypted)]);
}
// vuln-code-snippet end php_hardcoded_base64_key
