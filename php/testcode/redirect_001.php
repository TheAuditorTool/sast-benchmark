<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_header
function redirect001(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    header("Location: " . $url); // vuln-code-snippet vuln-line php_redirect_header
    return BenchmarkResponse::ok('Redirecting...');
}
// vuln-code-snippet end php_redirect_header
