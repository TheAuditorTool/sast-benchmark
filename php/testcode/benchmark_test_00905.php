<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00905(BenchmarkRequest $req): BenchmarkResponse {
    $str = $req->param('value');
    assert(strlen($str) > 0);
    return BenchmarkResponse::ok('non-empty: ' . strlen($str));
}
