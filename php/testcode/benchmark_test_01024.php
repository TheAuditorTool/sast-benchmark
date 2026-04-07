<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01024(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    assert(is_array(json_decode($data, true)));
    $parsed = json_decode($data, true);
    return BenchmarkResponse::ok(count($parsed) . ' items');
}
