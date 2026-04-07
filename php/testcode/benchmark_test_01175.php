<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01175(BenchmarkRequest $req): BenchmarkResponse {
    $logName = $req->param('log');
    $lines = file('/var/log/app/' . $logName);
    return BenchmarkResponse::json($lines);
}
