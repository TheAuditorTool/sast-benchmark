<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01195(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $output = shell_exec('ping -c 3 ' . escapeshellarg($host));
    return BenchmarkResponse::ok($output);
}
