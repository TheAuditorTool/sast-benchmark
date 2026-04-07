<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_printf_format_string
function xss026(BenchmarkRequest $req): BenchmarkResponse {
    $userInput = $req->param('name');
    ob_start();
    printf("%s", $userInput); // vuln-code-snippet vuln-line php_xss_printf_format_string
    $html = ob_get_clean();
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_printf_format_string
