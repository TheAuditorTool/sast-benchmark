<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_fnv1a32_password
function weakhash021(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('fnv1a32', $pass); // vuln-code-snippet vuln-line php_weakhash_fnv1a32_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_fnv1a32_password
