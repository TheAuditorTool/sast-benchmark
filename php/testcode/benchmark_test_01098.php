<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01098(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $hashed = hash('sha256', $data);
    return BenchmarkResponse::json(['hash' => $hashed]);
}
