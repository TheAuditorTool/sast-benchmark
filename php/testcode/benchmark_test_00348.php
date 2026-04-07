<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00348(BenchmarkRequest $req): BenchmarkResponse {
    $output = [];
    exec("ls -la /var/www", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
