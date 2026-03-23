<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_twig_raw
function xss_twig_raw(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');

    // Simulates Twig {{ input|raw }} -- raw filter bypasses escaping
    $html = '<p>' . $input . '</p>'; // vuln-code-snippet vuln-line php_xss_twig_raw

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_twig_raw
