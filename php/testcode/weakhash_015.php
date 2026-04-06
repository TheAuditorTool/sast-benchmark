<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sodium_generic
function weakhash015(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $hash = sodium_crypto_generichash($data); // vuln-code-snippet safe-line php_weakhash_sodium_generic
    return BenchmarkResponse::json(['hash' => bin2hex($hash)]);
}
// vuln-code-snippet end php_weakhash_sodium_generic
