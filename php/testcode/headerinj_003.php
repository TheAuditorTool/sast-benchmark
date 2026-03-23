<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_custom_header
function headerinj003(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    header("X-User: " . $username); // vuln-code-snippet vuln-line php_headerinj_custom_header
    return BenchmarkResponse::ok('Header set');
}
// vuln-code-snippet end php_headerinj_custom_header
