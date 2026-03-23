<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_twig_default
function xss_twig_default(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');

    // Simulates Twig {{ input }} -- auto-escaped (htmlspecialchars by default)
    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_twig_default
    $html = '<p>' . $safe . '</p>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_twig_default
