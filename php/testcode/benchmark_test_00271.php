<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00271(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    return processId041((int)$id);
}

function processId(int $userId): BenchmarkResponse {
    if ($userId === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
