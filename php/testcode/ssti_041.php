<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_escape_plus_csp
function ssti041(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    header("Content-Security-Policy: default-src 'self'"); // vuln-code-snippet safe-line php_ssti_escape_plus_csp
    $safe = htmlspecialchars($input, ENT_QUOTES);
    echo "<h1>$safe</h1>";
    return BenchmarkResponse::ok($safe);
}
// vuln-code-snippet end php_ssti_escape_plus_csp
