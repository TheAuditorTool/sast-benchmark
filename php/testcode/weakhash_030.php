<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_custom_xor_security
function weakhash030(BenchmarkRequest $req): BenchmarkResponse {
    $s = $req->param('token');
    $hash = array_sum(array_map('ord', str_split($s))); // vuln-code-snippet vuln-line php_weakhash_custom_xor_security
    return BenchmarkResponse::ok((string)$hash);
}
// vuln-code-snippet end php_weakhash_custom_xor_security
