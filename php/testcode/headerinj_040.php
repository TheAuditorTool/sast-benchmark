<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_cors_allowlist
function headerinj040(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->header('Origin');
    $allowed = ['https://app.example.com', 'https://admin.example.com'];
    if (in_array($origin, $allowed, true)) {
        header('Access-Control-Allow-Origin: ' . $origin); // vuln-code-snippet safe-line php_headerinj_cors_allowlist
    }
    return BenchmarkResponse::ok('cors handled');
}
// vuln-code-snippet end php_headerinj_cors_allowlist
