<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00832(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('dir');
    $output = [];
    exec("ls " . escapeshellarg($input), $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
