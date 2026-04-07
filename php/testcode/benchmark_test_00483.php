<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00483(BenchmarkRequest $req): BenchmarkResponse {
    $ct = $req->header('Content-Type');
    if ($ct !== 'application/json') {
        return BenchmarkResponse::badRequest('content-type must be application/json');
    }
    $data = json_decode($req->bodyStr(), true);
    performStateChange($data['key'], $data['value']);
    return BenchmarkResponse::json(['ok' => true]);
}
