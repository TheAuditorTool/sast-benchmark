<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00121(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->resolveExternals = true;
    $doc->loadXML($xml);
    $value = $doc->getElementsByTagName('data')->item(0)->textContent;
    return BenchmarkResponse::ok($value);
}
