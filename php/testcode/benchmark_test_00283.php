<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00283(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = openssl_digest($data, 'sha384');
    return BenchmarkResponse::json(['hash' => $digest]);
}
