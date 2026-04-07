<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_filter_validate_int_strict
function typejuggling043(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    $n = filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => 1]]); // vuln-code-snippet safe-line php_tj_filter_validate_int_strict
    if ($n === false) {
        return BenchmarkResponse::badRequest('invalid');
    }
    if ($n === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
// vuln-code-snippet end php_tj_filter_validate_int_strict
