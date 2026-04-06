<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_hmac_signed
function redirect014(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    $expected = hash_hmac('sha256', $url, getenv('REDIRECT_SECRET'));
    if (!hash_equals($expected, $sig)) { // vuln-code-snippet safe-line php_redirect_hmac_signed
        return BenchmarkResponse::badRequest('Invalid signature');
    }
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_hmac_signed
