<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00672(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true, 512, JSON_THROW_ON_ERROR);
    if (!is_int($data['id'])) {
        return BenchmarkResponse::badRequest('invalid');
    }
    if ($data['id'] === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
