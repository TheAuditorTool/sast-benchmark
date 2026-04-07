<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_fragment_bypass
function redirect025(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (($parsed['host'] ?? '') === 'example.com') {
        header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_fragment_bypass
    }
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_fragment_bypass
