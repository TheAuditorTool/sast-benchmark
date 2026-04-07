<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00474(BenchmarkRequest $req): BenchmarkResponse {
    $requestId = $req->param('request_id');
    header(sprintf("X-Request-Id: %s", $requestId));
    return BenchmarkResponse::ok('Processed');
}
