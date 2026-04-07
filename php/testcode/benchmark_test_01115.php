<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01115(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('id');
    if (intval($input) === 1) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::badRequest('not found');
}
