<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00871(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml);
    return BenchmarkResponse::ok($dom->saveXML());
}
