<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_array_filter_cb
function codeinj013(BenchmarkRequest $req): BenchmarkResponse {
    $callback = $req->param('filter');
    $data = explode(',', $req->param('data'));
    $filtered = array_filter($data, $callback); // vuln-code-snippet vuln-line php_codeinj_array_filter_cb
    return BenchmarkResponse::json(array_values($filtered));
}
// vuln-code-snippet end php_codeinj_array_filter_cb
