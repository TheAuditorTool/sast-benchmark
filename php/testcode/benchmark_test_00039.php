<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00039(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(true);
    $dom = new DOMDocument();
    $dom->loadXML($req->bodyStr(), LIBXML_NONET);
    $dom->schemaValidate('/etc/app/schema.xsd');
    return BenchmarkResponse::ok($dom->saveXML());
}
