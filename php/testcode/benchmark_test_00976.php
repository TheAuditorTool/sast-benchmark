<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00976(BenchmarkRequest $req): BenchmarkResponse {
    $key = 'k';
    $data = $req->param('data');
    $out = $data ^ str_repeat($key, strlen($data));
    return BenchmarkResponse::ok(base64_encode($out));
}
