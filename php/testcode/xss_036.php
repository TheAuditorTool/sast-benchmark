<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_csp_header_defense
function xss036(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    header("Content-Security-Policy: default-src 'self'"); // vuln-code-snippet safe-line php_xss_csp_header_defense
    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8');
    return BenchmarkResponse::html("<p>$safe</p>");
}
// vuln-code-snippet end php_xss_csp_header_defense
