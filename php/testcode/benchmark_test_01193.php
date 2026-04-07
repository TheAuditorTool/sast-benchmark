<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01193(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $output = shell_exec('ping -c 3 ' . $host);
    return BenchmarkResponse::ok($output);
}
