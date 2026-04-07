<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00119(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('fields');
    $parts = explode(',', $input);
    $a = $parts[0] ?? 'default_a';
    $b = $parts[1] ?? 'default_b';
    $$a = 'overwritten';
    $$b = 'overwritten';
    return BenchmarkResponse::ok('Done');
}
