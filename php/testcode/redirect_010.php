<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_double_encode
function redirect010(BenchmarkRequest $req): BenchmarkResponse {
    $url = urldecode($req->param('url'));
    header("Location: " . $url); // vuln-code-snippet vuln-line php_redirect_double_encode
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_double_encode
