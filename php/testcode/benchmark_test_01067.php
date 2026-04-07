<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01067(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_PARSEHUGE | LIBXML_NONET);
    return BenchmarkResponse::ok($dom->saveXML());
}
