<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00896(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
