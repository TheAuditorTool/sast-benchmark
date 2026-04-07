<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00691(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(false);
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
