<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00992(BenchmarkRequest $req): BenchmarkResponse {
    $direction = $req->param('direction');
    $arr = [3, 1, 4, 1, 5, 9, 2, 6];
    $comparatorCode = 'return ' . $direction . ';';
    eval('$cmp = function($a, $b) { ' . $comparatorCode . ' };');
    usort($arr, $cmp);
    return BenchmarkResponse::ok(implode(',', $arr));
}
