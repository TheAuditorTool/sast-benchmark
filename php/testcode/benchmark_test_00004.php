<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00004(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    $n = filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => 1]]);
    if ($n === false) {
        return BenchmarkResponse::badRequest('invalid');
    }
    if ($n === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
