<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00935(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml);
    $value = $dom->documentElement->textContent;
    return BenchmarkResponse::ok($value);
}
