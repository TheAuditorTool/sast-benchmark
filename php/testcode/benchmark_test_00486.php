<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00486(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_DTDLOAD);
    return BenchmarkResponse::ok($dom->saveXML());
}
