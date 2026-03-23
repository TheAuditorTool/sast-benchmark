<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_strip_tags
function xss_strip_tags(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('bio');

    $safe = strip_tags($input); // vuln-code-snippet safe-line php_xss_strip_tags
    $html = '<p>' . $safe . '</p>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_strip_tags
