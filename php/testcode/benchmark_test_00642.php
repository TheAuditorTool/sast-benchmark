<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00642(BenchmarkRequest $req): BenchmarkResponse {
    $a = $req->param('a');
    $b = $req->param('b');
    if ($a == $b) {
        return BenchmarkResponse::ok('same');
    }
    return BenchmarkResponse::ok('different');
}
