<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_signed_timestamp_expiry
function redirect038(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ts = $req->param('ts');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url . $ts, getenv('SECRET')), $sig) || time() - $ts > 300) {
        return BenchmarkResponse::badRequest('invalid');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_signed_timestamp_expiry
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_signed_timestamp_expiry
