<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00006(BenchmarkRequest $req): BenchmarkResponse {
    $outputFile = escapeshellarg('/tmp/report_' . bin2hex(random_bytes(8)) . '.txt');
    $output = [];
    exec("df -h > $outputFile", $output);
    return BenchmarkResponse::ok('done');
}
