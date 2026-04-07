<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01230(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml);
    $items = $dom->getElementsByTagName('item');
    $results = [];
    foreach ($items as $item) {
        $results[] = $item->nodeValue;
    }
    return BenchmarkResponse::json($results);
}
