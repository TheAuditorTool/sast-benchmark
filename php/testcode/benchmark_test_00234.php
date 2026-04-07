<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00234(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('dir');
    $output = [];
    exec("ls " . $input, $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
