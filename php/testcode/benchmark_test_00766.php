<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00766(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = yaml_parse($body);
    return BenchmarkResponse::json(is_array($data) ? $data : []);
}
