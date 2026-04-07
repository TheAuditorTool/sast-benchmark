<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00938(BenchmarkRequest $req): BenchmarkResponse {
    $key = basename($req->param('key'));
    $xml = file_get_contents('s3://trusted-bucket/' . $key);
    libxml_disable_entity_loader(true);
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
