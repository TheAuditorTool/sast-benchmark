<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00106(BenchmarkRequest $req): BenchmarkResponse {
    $xml = '<root><item>value</item></root>';
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data->item);
}
