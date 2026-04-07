<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_double_encoding_bypass
function redirect026(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $decoded = urldecode($url);
    if (strpos($decoded, '//') === false) {
        header('Location: ' . $decoded); // vuln-code-snippet vuln-line php_redirect_double_encoding_bypass
    }
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_double_encoding_bypass
