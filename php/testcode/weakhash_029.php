<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md4_password
function weakhash029(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('md4', $pass); // vuln-code-snippet vuln-line php_weakhash_md4_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_md4_password
