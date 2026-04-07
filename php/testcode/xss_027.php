<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_data_uri_src
function xss027(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $html = "<img src=\"data:text/html,$input\">"; // vuln-code-snippet vuln-line php_xss_data_uri_src
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_data_uri_src
