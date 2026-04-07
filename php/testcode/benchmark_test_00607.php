<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00607(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data);
    return BenchmarkResponse::json(['result' => $obj]);
}
