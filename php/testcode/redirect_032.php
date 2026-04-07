<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_same_origin_slash
function redirect032(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (strpos($url, '/') !== 0 || strpos($url, '//') === 0) {
        return BenchmarkResponse::badRequest('not relative');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_same_origin_slash
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_same_origin_slash
