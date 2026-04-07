<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00245(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $handle = popen("sort " . $filename, "r");
    $output = fread($handle, 8192);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
