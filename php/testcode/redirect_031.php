<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_301_open
function redirect031(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    http_response_code(301);
    header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_301_open
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_301_open
