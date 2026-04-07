<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00175(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    if ($data === null) {
        return BenchmarkResponse::badRequest('invalid json');
    }
    $name = $data['name'] ?? 'unknown';
    return BenchmarkResponse::ok("hello $name");
}
