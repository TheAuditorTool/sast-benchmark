<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_nl2br_escaped
function xss032(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('text');
    $safe = nl2br(htmlspecialchars($input, ENT_QUOTES, 'UTF-8')); // vuln-code-snippet safe-line php_xss_nl2br_escaped
    return BenchmarkResponse::html("<p>$safe</p>");
}
// vuln-code-snippet end php_xss_nl2br_escaped
