<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00499(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $handle = popen("sort " . escapeshellarg($filename), "r");
    $output = fread($handle, 8192);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
