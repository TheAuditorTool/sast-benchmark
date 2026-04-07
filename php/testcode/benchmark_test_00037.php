<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00037(BenchmarkRequest $req): BenchmarkResponse {
    $date = date('Y-m-d');
    $output = [];
    exec("find /var/logs -name 'app-$date.log'", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
