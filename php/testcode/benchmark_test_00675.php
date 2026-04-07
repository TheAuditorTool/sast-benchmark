<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00675(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = simplexml_load_string($xml);
    if ($doc === false) {
        return BenchmarkResponse::badRequest('invalid xml');
    }
    return BenchmarkResponse::ok((string) $doc->name);
}
