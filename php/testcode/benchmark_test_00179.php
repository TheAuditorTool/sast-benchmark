<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00179(BenchmarkRequest $req): BenchmarkResponse {
    $count = $req->param('count');
    $output = [];
    exec("ping -c " . intval($count) . " 8.8.8.8", $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
