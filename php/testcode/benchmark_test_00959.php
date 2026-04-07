<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00959(BenchmarkRequest $req): BenchmarkResponse {
    $tracking = $req->header('X-Tracking-Id');
    header("X-Tracking-Id: " . $tracking);
    return BenchmarkResponse::ok('Tracked');
}
