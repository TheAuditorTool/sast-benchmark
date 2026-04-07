<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00612(BenchmarkRequest $req): BenchmarkResponse {
    $a = $req->param('a');
    $b = $req->param('b');
    if (strcmp($a, $b) === 0) {
        return BenchmarkResponse::ok('equal');
    }
    return BenchmarkResponse::ok('not equal');
}
