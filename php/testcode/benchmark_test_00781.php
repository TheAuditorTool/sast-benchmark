<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00781(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml);
    $name = $doc->getElementsByTagName('name')->item(0)->textContent;
    return BenchmarkResponse::ok($name);
}
