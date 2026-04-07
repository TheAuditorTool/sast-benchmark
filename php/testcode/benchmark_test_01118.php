<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01118(BenchmarkRequest $req): BenchmarkResponse {
    $ct = $req->header('Content-Type');
    if (strpos($ct, 'application/json') === false) {
        return BenchmarkResponse::badRequest('json only');
    }
    $data = json_decode($req->bodyStr(), true);
    return BenchmarkResponse::ok(json_encode($data));
}
