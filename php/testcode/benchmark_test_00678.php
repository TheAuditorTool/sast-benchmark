<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00678(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $cmd = sprintf("find %s -name '*.txt'", $dir);
    $output = [];
    exec($cmd, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
