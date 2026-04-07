<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00470(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NONET);
    $value = $doc->getElementsByTagName('data')->item(0)->textContent;
    return BenchmarkResponse::ok($value);
}
