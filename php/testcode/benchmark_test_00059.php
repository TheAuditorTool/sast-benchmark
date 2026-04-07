<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00059(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if ($id + 0 == 1) {
        return BenchmarkResponse::ok('item one');
    }
    return BenchmarkResponse::badRequest('not found');
}
