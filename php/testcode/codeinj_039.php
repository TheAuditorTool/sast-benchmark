<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_assert_bool_php8
function codeinj039(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    assert(is_array(json_decode($data, true))); // vuln-code-snippet safe-line php_codeinj_assert_bool_php8
    $parsed = json_decode($data, true);
    return BenchmarkResponse::ok(count($parsed) . ' items');
}
// vuln-code-snippet end php_codeinj_assert_bool_php8
