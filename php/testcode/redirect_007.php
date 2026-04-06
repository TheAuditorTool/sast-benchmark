<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_protocol_relative
function redirect007(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('next');
    header("Location: " . $url); // vuln-code-snippet vuln-line php_redirect_protocol_relative
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_protocol_relative
