<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_attribute_unquoted
function xss_attribute_unquoted(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('avatar');

    $html = '<img src=' . $input . ' alt="avatar">'; // vuln-code-snippet vuln-line php_xss_attribute_unquoted

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_attribute_unquoted
