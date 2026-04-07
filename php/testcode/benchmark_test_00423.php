<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00423(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(true);
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
