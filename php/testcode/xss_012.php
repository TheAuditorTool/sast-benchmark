<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_esc_html_wp
function xss_esc_html_wp(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('title');

    $safe = esc_html($input); // vuln-code-snippet safe-line php_xss_esc_html_wp
    $html = '<h1>' . $safe . '</h1>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_esc_html_wp
