<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00914(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $target = $req->param('target');
    $output = [];
    exec(escapeshellarg($cmd) . ' ' . $target, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
