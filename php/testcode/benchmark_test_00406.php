<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00406(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = hash('md4', $data);
    return BenchmarkResponse::json(['hash' => $digest]);
}
