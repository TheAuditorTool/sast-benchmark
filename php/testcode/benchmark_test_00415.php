<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00415(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = hash('sha3-256', $data);
    return BenchmarkResponse::json(['hash' => $digest]);
}
