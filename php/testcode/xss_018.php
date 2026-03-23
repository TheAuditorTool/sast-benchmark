<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_blade_escaped
function xss_blade_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('content');

    // Simulates Blade {{ $input }} -- auto-escaped with htmlspecialchars
    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_blade_escaped
    $html = '<div>' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_blade_escaped
