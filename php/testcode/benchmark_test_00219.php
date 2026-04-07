<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00219(BenchmarkRequest $req): BenchmarkResponse {
    $args = $req->param('args');
    $argArray = explode(',', $args);
    $cmd = implode(' ', $argArray);
    $output = [];
    exec($cmd, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
