<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_print_concat
function xss_print_concat(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('user');

    $body = '<div>Hello ' . $input . '</div>'; // vuln-code-snippet vuln-line php_xss_print_concat

    return BenchmarkResponse::html($body);
}
// vuln-code-snippet end php_xss_print_concat
