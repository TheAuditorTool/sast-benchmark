<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00063(BenchmarkRequest $req): BenchmarkResponse {
    $ip = $req->param('ip');
    if (!filter_var($ip, FILTER_VALIDATE_IP)) {
        return BenchmarkResponse::badRequest('invalid ip');
    }
    $output = [];
    exec("ping -c 3 " . $ip, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
