<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01231(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    libxml_disable_entity_loader(true);
    $doc = simplexml_load_string($xml, 'SimpleXMLElement', LIBXML_NONET);
    $title = (string)$doc->title;
    return BenchmarkResponse::ok($title);
}
