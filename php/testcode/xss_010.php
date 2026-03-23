<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_json_encode
function xss_json_encode(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('config');

    $encoded = json_encode($input); // vuln-code-snippet safe-line php_xss_json_encode
    $html = '<script>var x = ' . $encoded . ';</script>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_json_encode
