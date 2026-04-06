<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_openssl_seal
function weakcipher017(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $pubKey = openssl_pkey_get_public(getenv('RSA_PUBLIC_KEY'));
    $sealed = '';
    $envKeys = [];
    $iv = '';
    openssl_seal($data, $sealed, $envKeys, [$pubKey], 'aes-256-cbc', $iv); // vuln-code-snippet safe-line php_weakcipher_openssl_seal
    return BenchmarkResponse::json(['sealed' => base64_encode($sealed), 'key' => base64_encode($envKeys[0])]);
}
// vuln-code-snippet end php_weakcipher_openssl_seal
