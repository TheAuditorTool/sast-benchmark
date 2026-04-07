<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00511(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    if ($val == false) {
        return BenchmarkResponse::ok('empty');
    }
    return BenchmarkResponse::ok('has value');
}
