<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00584(BenchmarkRequest $req): BenchmarkResponse {
    $json = $req->bodyStr();
    $data = json_decode($json, true);
    return BenchmarkResponse::ok(json_encode($data));
}
