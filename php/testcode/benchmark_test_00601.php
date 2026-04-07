<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00601(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NOENT | LIBXML_NONET);
    $name = $doc->getElementsByTagName('name')->item(0)->textContent ?? '';
    return BenchmarkResponse::ok($name);
}
