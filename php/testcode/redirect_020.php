<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_base_path_bypass
function redirect020(BenchmarkRequest $req): BenchmarkResponse {
    $base = 'https://example.com';
    $path = $req->param('path');
    header('Location: ' . $base . $path); // vuln-code-snippet vuln-line php_redirect_base_path_bypass
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_base_path_bypass
