<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00307(BenchmarkRequest $req): BenchmarkResponse {
    $comparator = $req->post('comparator');
    $arr = [3, 1, 2];
    usort($arr, $comparator);
    return BenchmarkResponse::ok(implode(',', $arr));
}
