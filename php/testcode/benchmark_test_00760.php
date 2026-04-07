<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00760(BenchmarkRequest $req): BenchmarkResponse {
    $s = $req->param('token');
    $hash = array_sum(array_map('ord', str_split($s)));
    return BenchmarkResponse::ok((string)$hash);
}
