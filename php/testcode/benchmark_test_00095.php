<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00095(BenchmarkRequest $req): BenchmarkResponse {
    header('Access-Control-Allow-Origin: *');
    header('Access-Control-Allow-Headers: X-Custom-Token');
    $token = $req->header('X-Custom-Token');
    if (empty($token)) {
        return BenchmarkResponse::badRequest('missing token');
    }
    performAction($req->bodyStr());
    return BenchmarkResponse::json(['ok' => true]);
}
