<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00771(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NONET | LIBXML_DTDATTR);
    return BenchmarkResponse::ok($dom->saveXML());
}
