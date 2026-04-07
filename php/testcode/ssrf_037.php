<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_signed_token_url
function ssrf037(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $sig = $req->param('sig');
    if (!hash_equals(hash_hmac('sha256', $url, getenv('FETCH_SECRET')), $sig)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_signed_token_url
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_signed_token_url
