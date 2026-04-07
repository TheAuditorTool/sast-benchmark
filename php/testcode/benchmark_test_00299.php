<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00299(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['user' => $req->param('name'), 'ts' => time()];
    $json = json_encode($data, JSON_HEX_TAG | JSON_HEX_AMP);
    return BenchmarkResponse::ok($json);
}
