<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_array_filter_string
function codeinj025(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $arr = ['alpha', 'beta', 'gamma', 'delta'];
    $filtered = array_filter($arr, $fn); // vuln-code-snippet vuln-line php_codeinj_array_filter_string
    return BenchmarkResponse::ok(implode(',', $filtered));
}
// vuln-code-snippet end php_codeinj_array_filter_string
