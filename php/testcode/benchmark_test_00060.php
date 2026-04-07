<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00060(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if ((int)$id === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::badRequest('not found');
}
