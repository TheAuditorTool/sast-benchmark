<?php
require_once __DIR__ . '/shared.php';

define('REPORT_CMD', '/usr/local/bin/generate-report');

function benchmarkTest00103(BenchmarkRequest $req): BenchmarkResponse {
    $output = [];
    exec(REPORT_CMD . ' --format pdf', $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
