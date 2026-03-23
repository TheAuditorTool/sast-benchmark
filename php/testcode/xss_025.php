<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_printf_format
function xss_printf_format(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('label');

    $html = sprintf('<p>%s</p>', $input); // vuln-code-snippet vuln-line php_xss_printf_format

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_printf_format
