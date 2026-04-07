<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00081(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $arg = $req->param('arg');
    $output = [];
    exec(escapeshellarg($cmd) . ' ' . $arg, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
