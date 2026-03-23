<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_js
function redirect005(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $html = '<script>window.location="' . $url . '";</script>'; // vuln-code-snippet vuln-line php_redirect_js
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_redirect_js
