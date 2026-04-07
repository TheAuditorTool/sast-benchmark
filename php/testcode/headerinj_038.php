<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_sanitize_url_location
function headerinj038(BenchmarkRequest $req): BenchmarkResponse {
    $url = filter_var($req->param('url'), FILTER_SANITIZE_URL); // vuln-code-snippet safe-line php_headerinj_sanitize_url_location
    if (strpos($url, 'https://') !== 0) {
        return BenchmarkResponse::badRequest('bad url');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_headerinj_sanitize_url_location
