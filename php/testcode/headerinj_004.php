<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_urlencode
function headerinj004(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    header("X-User: " . urlencode($username)); // vuln-code-snippet safe-line php_headerinj_urlencode
    return BenchmarkResponse::ok('Header set');
}
// vuln-code-snippet end php_headerinj_urlencode
