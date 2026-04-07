<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01136(BenchmarkRequest $req): BenchmarkResponse {
    $n = $req->param('n');
    $literal = json_encode($n);
    $val = eval('return ' . $literal . ';');
    return BenchmarkResponse::ok((string) $val);
}
