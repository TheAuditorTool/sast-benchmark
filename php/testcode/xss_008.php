<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_attribute_escaped
function xss_attribute_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('avatar');

    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_attribute_escaped
    $html = '<img src="' . $safe . '" alt="avatar">';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_attribute_escaped
