<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_bytes_iv
function weakrand043(BenchmarkRequest $req): BenchmarkResponse {
    $iv = random_bytes(16); // vuln-code-snippet safe-line php_weakrand_random_bytes_iv
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', getenv('KEY'), 0, $iv);
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakrand_random_bytes_iv
