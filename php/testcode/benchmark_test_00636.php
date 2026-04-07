<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00636(BenchmarkRequest $req): BenchmarkResponse {
    $value = $req->param('value');
    $fn = function ($x) { return $x * 2; };
    $result = $fn((int) $value);
    return BenchmarkResponse::ok("Result: " . $result);
}
