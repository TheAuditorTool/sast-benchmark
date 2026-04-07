<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00142(BenchmarkRequest $req): BenchmarkResponse {
    $verbose = (bool)getenv('DEBUG_VERBOSE');
    $flag = $verbose ? '-v' : '';
    $output = [];
    exec("/usr/local/bin/check-service $flag", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
