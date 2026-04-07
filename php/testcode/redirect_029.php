<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_decode_after_validate
function redirect029(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (parse_url($url, PHP_URL_HOST) === 'example.com') {
        header('Location: ' . urldecode($url)); // vuln-code-snippet vuln-line php_redirect_decode_after_validate
    }
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_decode_after_validate
