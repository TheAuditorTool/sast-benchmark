<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00776(BenchmarkRequest $req): BenchmarkResponse {
    $response = new BenchmarkResponse(200, 'ok');
    $safeValue = $response->withHeader('X-Custom', $req->param('val'));
    return BenchmarkResponse::ok('header set via psr7');
}
