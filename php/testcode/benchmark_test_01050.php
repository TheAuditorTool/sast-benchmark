<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01050(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    libxml_use_internal_errors(true);
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
