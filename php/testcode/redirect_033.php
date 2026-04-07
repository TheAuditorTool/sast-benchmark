<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_hmac_signed_url
function redirect033(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url, getenv('REDIRECT_SECRET')), $sig)) {
        return BenchmarkResponse::badRequest('invalid sig');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_hmac_signed_url
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_hmac_signed_url
