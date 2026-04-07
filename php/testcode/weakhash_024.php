<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_murmur_security
function weakhash024(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = hash('murmur3a', $data); // vuln-code-snippet vuln-line php_weakhash_murmur_security
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_murmur_security
