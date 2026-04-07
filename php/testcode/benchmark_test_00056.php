<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00056(BenchmarkRequest $req): BenchmarkResponse {
    $compressed = $req->bodyStr();
    $xml = gzinflate($compressed);
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
