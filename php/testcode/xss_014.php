<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_esc_url_wp
function xss_esc_url_wp(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');

    $safe = esc_url($input); // vuln-code-snippet safe-line php_xss_esc_url_wp
    $html = '<a href="' . $safe . '">Click here</a>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_esc_url_wp
