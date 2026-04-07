<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_random_iv_per_msg
function weakcipher046(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(12); // vuln-code-snippet safe-line php_weakcipher_random_iv_per_msg
    $tag = '';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, $iv, $tag);
    $result = base64_encode($iv . $tag . $enc);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_weakcipher_random_iv_per_msg
