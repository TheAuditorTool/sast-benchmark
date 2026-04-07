<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00915(BenchmarkRequest $req): BenchmarkResponse {
    $yamlStr = $req->bodyStr();
    $data = yaml_parse($yamlStr);
    return BenchmarkResponse::json($data);
}
