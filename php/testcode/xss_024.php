<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_wp_kses
function xss_wp_kses(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('content');

    $safe = wp_kses_post($input); // vuln-code-snippet safe-line php_xss_wp_kses
    $html = '<article>' . $safe . '</article>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_wp_kses
