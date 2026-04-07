<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00544(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NOENT | LIBXML_NONET | LIBXML_NOERROR);
    return BenchmarkResponse::ok($dom->saveXML());
}
