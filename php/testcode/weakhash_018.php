<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_openssl_digest
function weakhash018(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = openssl_digest($data, 'sha384'); // vuln-code-snippet safe-line php_weakhash_openssl_digest
    return BenchmarkResponse::json(['hash' => $digest]);
}
// vuln-code-snippet end php_weakhash_openssl_digest
