<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00011(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $value = $req->param('value');
    $config = [];
    $config[$key] = $value;
    return BenchmarkResponse::json($config);
}
