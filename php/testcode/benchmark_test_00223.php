<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00223(BenchmarkRequest $req): BenchmarkResponse {
    $n = $req->param('n');
    $formatted = sprintf('%d', $n);
    $output = [];
    exec("seq $formatted", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
