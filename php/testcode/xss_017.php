<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_blade_raw
function xss_blade_raw(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('content');

    // Simulates Blade {!! $input !!} -- raw, unescaped output
    $html = '<div>' . $input . '</div>'; // vuln-code-snippet vuln-line php_xss_blade_raw

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_blade_raw
