<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01232(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NONET | LIBXML_DTDLOAD);
    $items = $dom->getElementsByTagName('item');
    $results = [];
    foreach ($items as $item) {
        $results[] = $item->nodeValue;
    }
    return BenchmarkResponse::json($results);
}
