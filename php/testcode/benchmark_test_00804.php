<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00804(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $arr = ['alpha', 'beta', 'gamma', 'delta'];
    $filtered = array_filter($arr, $fn);
    return BenchmarkResponse::ok(implode(',', $filtered));
}
