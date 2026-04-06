<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_xor
function weakcipher011(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('ENCRYPTION_KEY');
    $result = '';
    for ($i = 0; $i < strlen($data); $i++) {
        $result .= $data[$i] ^ $key[$i % strlen($key)]; // vuln-code-snippet vuln-line php_weakcipher_xor
    }
    return BenchmarkResponse::json(['ciphertext' => base64_encode($result)]);
}
// vuln-code-snippet end php_weakcipher_xor
