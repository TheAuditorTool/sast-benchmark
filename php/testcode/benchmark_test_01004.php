<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01004(BenchmarkRequest $req): BenchmarkResponse {
    $yamlStr = $req->bodyStr();
    $ndocs = 0;
    $data = yaml_parse($yamlStr, 0, $ndocs, []);
    return BenchmarkResponse::json($data);
}
