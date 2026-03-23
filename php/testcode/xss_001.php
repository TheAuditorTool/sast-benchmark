<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_echo_unescaped
function xss_echo_unescaped(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');

    $html = '<html><body><h1>Hello, ' . $name . '!</h1></body></html>'; // vuln-code-snippet vuln-line php_xss_echo_unescaped

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_echo_unescaped
