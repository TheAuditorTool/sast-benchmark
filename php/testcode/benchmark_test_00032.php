<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00032(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NONET);
    $dom->schemaValidate('/etc/app/schema.xsd');
    return BenchmarkResponse::ok($dom->saveXML());
}
