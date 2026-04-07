<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00336(BenchmarkRequest $req): BenchmarkResponse {
    $ip = $req->param('ip');
    if (!preg_match('/^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/', $ip)) {
        return BenchmarkResponse::badRequest('invalid ip');
    }
    $output = [];
    exec("ping -c 1 $ip", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
