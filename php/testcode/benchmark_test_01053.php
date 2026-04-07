<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01053(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('path');
    putenv("PATH=$input");
    $output = [];
    exec('ls', $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
