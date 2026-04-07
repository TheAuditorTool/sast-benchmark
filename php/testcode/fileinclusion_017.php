<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_filter_wrapper
function fileinclusion017(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $path = 'php://filter/read=convert.base64-encode/resource=' . $file;
    include $path; // vuln-code-snippet vuln-line php_fi_filter_wrapper
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_filter_wrapper
