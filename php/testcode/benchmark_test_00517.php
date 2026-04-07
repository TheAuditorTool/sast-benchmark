<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00517(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $data = explode(',', $req->param('data'));
    $result = array_map($fn, $data);
    return BenchmarkResponse::json($result);
}
