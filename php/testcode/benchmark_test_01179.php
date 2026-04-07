<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01179(BenchmarkRequest $req): BenchmarkResponse {
    $logName = $req->param('log');
    $clean = basename($logName);
    $lines = file('/var/log/app/' . $clean);
    return BenchmarkResponse::json($lines);
}
