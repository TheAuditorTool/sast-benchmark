<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_href_javascript
function xss_href_javascript(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');

    $html = '<a href="' . $input . '">Click here</a>'; // vuln-code-snippet vuln-line php_xss_href_javascript

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_href_javascript
